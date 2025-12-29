use crate::operations::{Header, Operations};

/// Represents a parsed RemoteCompose document with a tree structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// The document header.
    pub header: Header,
    /// The root level nodes of the document.
    pub root: Vec<Operations>,
}

#[derive(Debug, Clone, PartialEq)]
enum BuilderItem {
    Root(Vec<Operations>),
    Container { op: Operations, is_content: bool },
    ActionList(Operations),
    ContentMarker,
}

impl BuilderItem {
    fn name(&self) -> String {
        match self {
            BuilderItem::Root(_) => "Root".to_string(),
            BuilderItem::Container { op, .. } => format!("{:?}", op),
            BuilderItem::ActionList(op) => format!("{:?}", op),
            BuilderItem::ContentMarker => "ContentMarker".to_string(),
        }
    }

    fn id(&self) -> Option<i32> {
        match self {
            BuilderItem::Container { op, .. } => match op {
                Operations::LayoutRoot { component_id, .. } => Some(*component_id),
                Operations::LayoutBox { component_id, .. } => Some(*component_id),
                Operations::LayoutRow { component_id, .. } => Some(*component_id),
                Operations::LayoutColumn { component_id, .. } => Some(*component_id),
                Operations::LayoutCanvas { component_id, .. } => Some(*component_id),
                Operations::LayoutText { component_id, .. } => Some(*component_id),
                Operations::LayoutImage { component_id, .. } => Some(*component_id),
                Operations::LayoutState { component_id, .. } => Some(*component_id),
                Operations::LayoutFitBox { .. } => None,
                Operations::LayoutCollapsibleRow { .. } => None,
                Operations::LayoutCollapsibleColumn { .. } => None,
                Operations::LayoutCompute { .. } => None,
                Operations::LayoutCanvasContent { component_id, .. } => Some(*component_id),
                _ => None,
            },
            BuilderItem::ContentMarker => None,
            _ => None,
        }
    }
}

pub struct DocumentBuilder {
    header: Option<Header>,
    stack: Vec<BuilderItem>,
    lenient: bool,
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            header: None,
            stack: vec![BuilderItem::Root(Vec::new())],
            lenient: false,
        }
    }

    pub fn with_lenient(mut self, lenient: bool) -> Self {
        self.lenient = lenient;
        self
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = Some(header);
    }

    pub fn push_op(&mut self, op: Operations, index: usize) -> Result<(), String> {
        // Special case: Header must be first and only once
        if let Operations::Header {
            major,
            minor,
            patch,
            ref metadata,
        } = op
        {
            if self.header.is_some() {
                return Err(format!("Duplicate Header at Op Index {}", index));
            }
            self.header = Some(Header {
                major,
                minor,
                patch,
                metadata: metadata.clone(),
            });
            return Ok(());
        }

        if self.header.is_none() {
            return Err(format!(
                "First operation must be Header. Found {:?} at Op Index {}",
                op, index
            ));
        }

        match op {
            Operations::LayoutContent { .. } => {
                if let Some(BuilderItem::Container { is_content, .. }) = self.stack.last_mut() {
                    *is_content = true;
                    self.stack.push(BuilderItem::ContentMarker);
                    Ok(())
                } else {
                    Err(self.error(
                        "LayoutContent must be inside a Container.".to_string(),
                        index,
                    ))
                }
            }
            Operations::ContainerEnd => {
                match self.stack.pop() {
                    Some(BuilderItem::ContentMarker) => Ok(()), // Just closed content
                    Some(finished_item) => {
                        let op_to_add = match finished_item {
                            BuilderItem::Container { op, .. } => op,
                            BuilderItem::ActionList(op) => op,
                            BuilderItem::Root(_) => {
                                return Err(self.error("Cannot pop root.".to_string(), index));
                            }
                            BuilderItem::ContentMarker => unreachable!(),
                        };
                        self.add_to_parent(op_to_add, index)
                    }
                    None => Err(self.error("Stack underflow.".to_string(), index)),
                }
            }
            _ => {
                if let Some(_) = op.as_container() {
                    let is_content = !op.has_modifiers();
                    self.stack.push(BuilderItem::Container {
                        op: op.clone(),
                        is_content,
                    });
                    Ok(())
                } else if let Some(_) = op.as_action_list() {
                    self.stack.push(BuilderItem::ActionList(op.clone()));
                    Ok(())
                } else {
                    self.add_to_current(op, index)
                }
            }
        }
    }

    fn add_to_current(&mut self, op: Operations, index: usize) -> Result<(), String> {
        let len = self.stack.len();
        match self.stack.last_mut() {
            Some(BuilderItem::Root(nodes)) => {
                nodes.push(op);
                Ok(())
            }
            Some(BuilderItem::ContentMarker) => {
                if len < 2 {
                    return Err(
                        self.error("ContentMarker without container parent".to_string(), index)
                    );
                }
                if let BuilderItem::Container {
                    op: ref mut parent_op,
                    ..
                } = self.stack[len - 2]
                {
                    parent_op
                        .as_container_mut()
                        .unwrap()
                        .children_mut()
                        .push(op);
                    Ok(())
                } else {
                    Err(self.error("ContentMarker not below container".to_string(), index))
                }
            }
            Some(BuilderItem::Container {
                op: ref mut parent_op,
                is_content,
            }) => {
                let container = parent_op.as_container_mut().unwrap();
                if *is_content {
                    container.children_mut().push(op);
                } else {
                    container.modifiers_mut().push(op);
                }
                Ok(())
            }
            Some(BuilderItem::ActionList(ref mut parent_op)) => {
                let parent_actions = parent_op.as_action_list_mut().unwrap();
                parent_actions.actions_mut().push(op);
                Ok(())
            }
            None => Err(self.error("Stack empty".to_string(), index)),
        }
    }

    fn add_to_parent(&mut self, finished_op: Operations, index: usize) -> Result<(), String> {
        self.add_to_current(finished_op, index)
    }

    pub fn finish(mut self) -> Result<Document, String> {
        let header = self.header.take().ok_or("Missing header")?;

        if self.lenient {
            while self.stack.len() > 1 {
                let top = self.stack.pop();
                match top {
                    Some(BuilderItem::Container { op, .. }) | Some(BuilderItem::ActionList(op)) => {
                        // Attempt to add the unclosed operation to its parent (the new top of stack)
                        if let Err(_) = self.add_to_current(op, 0) {
                            // Ignore errors during lenient unwinding
                        }
                    }
                    Some(BuilderItem::ContentMarker) => {
                        // Just a marker, removing it exposes the parent which is what we want
                    }
                    _ => {}
                }
            }
        }

        if self.stack.len() != 1 {
            return Err(self.error("Unclosed blocks".to_string(), 0));
        }

        if let BuilderItem::Root(root) = self.stack.pop().unwrap() {
            Ok(Document { header, root })
        } else {
            Err("Root item missing or corrupted".to_string())
        }
    }

    fn error(&self, msg: String, index: usize) -> String {
        let mut out = format!("{} at Op Index {}.\n", msg, index);
        out.push_str("Current Stack IDs: [");
        for (i, item) in self.stack.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            if let Some(id) = item.id() {
                out.push_str(&id.to_string());
            } else {
                out.push_str("None");
            }
        }
        out.push_str("]\n");
        out.push_str("Minimal Tree Structure:\n");
        out.push_str(&self.dump_stack(0));
        out
    }

    fn dump_stack(&self, depth: usize) -> String {
        let mut out = String::new();
        for item in &self.stack {
            let indent = "  ".repeat(depth);
            let id_str = item.id().map(|id| format!(" ({})", id)).unwrap_or_default();
            out.push_str(&format!("{}- {}{}\n", indent, item.name(), id_str));
        }
        out
    }
}

impl Document {
    /// Parses a stream of bytes into a Document tree.
    pub fn parse(data: &[u8]) -> Result<Document, String> {
        let stream = OperationStream::new(data);
        Document::build(stream)
    }

    /// Parses a stream of bytes into a Document tree with lenient mode.
    pub fn parse_lenient(data: &[u8]) -> Result<Document, String> {
        let stream = OperationStream::new(data);
        Document::build_lenient(stream)
    }

    /// Builds a Document tree from a list of operations.
    pub fn from_operations(
        operations: impl IntoIterator<Item = Operations>,
    ) -> Result<Document, String> {
        Self::build(operations.into_iter().map(Ok))
    }

    /// Builds a Document tree from a list of operations with lenient mode.
    pub fn from_operations_lenient(
        operations: impl IntoIterator<Item = Operations>,
    ) -> Result<Document, String> {
        Self::build_lenient(operations.into_iter().map(Ok))
    }

    /// Builds a Document tree from a stream of operations.
    pub fn build(
        operations: impl Iterator<Item = Result<Operations, String>>,
    ) -> Result<Document, String> {
        let mut builder = DocumentBuilder::new();

        for (index, op_res) in operations.enumerate() {
            let op = op_res.map_err(|e| format!("Read error at Op Index {}: {}", index, e))?;
            builder.push_op(op, index)?;
        }

        builder.finish()
    }

    /// Builds a Document tree from a stream of operations with lenient mode.
    /// In lenient mode, unclosed blocks are ignored at the end of the stream.
    pub fn build_lenient(
        operations: impl Iterator<Item = Result<Operations, String>>,
    ) -> Result<Document, String> {
        let mut builder = DocumentBuilder::new().with_lenient(true);

        for (index, op_res) in operations.enumerate() {
            let op = op_res.map_err(|e| format!("Read error at Op Index {}: {}", index, e))?;
            builder.push_op(op, index)?;
        }

        builder.finish()
    }
}

pub struct OperationStream<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> OperationStream<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
}

impl<'a> Iterator for OperationStream<'a> {
    type Item = Result<Operations, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.data.len() {
            return None;
        }
        match Operations::read(&self.data[self.offset..]) {
            Ok((op, len)) => {
                self.offset += len;
                Some(Ok(op))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
