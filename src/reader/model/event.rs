use derive_more::From;

use crate::event::{Event as ReifiedEvent, XmlVersion};
use crate::namespace::Namespace;
use crate::reader::data::{BufSlice, Buffer};

use super::{Attribute, Name};

#[derive(Debug, Clone)]
pub enum Event {
    StartDocument {
        version: XmlVersion,
        encoding: BufSlice,
        standalone: Option<bool>,
    },

    EndDocument,

    DoctypeDeclaration {
        content: BufSlice,
    },

    ProcessingInstruction {
        name: BufSlice,
        data: Option<BufSlice>,
    },

    StartElement {
        name: Name,
        // TODO: consider using SmallVec
        attributes: Vec<Attribute>,
    },

    EndElement {
        name: Name,
    },

    CData(BufSlice),

    Comment(BufSlice),

    Text(BufSlice),

    Whitespace(BufSlice),
}

impl Event {
    pub fn start_document(version: XmlVersion, encoding: impl Into<BufSlice>, standalone: Option<bool>) -> Event {
        Event::StartDocument {
            version,
            encoding: encoding.into(),
            standalone,
        }
    }

    pub fn end_document() -> Event {
        Event::EndDocument
    }

    pub fn doctype_declaration(content: impl Into<BufSlice>) -> Event {
        Event::DoctypeDeclaration {
            content: content.into(),
        }
    }

    pub fn processing_instruction(name: impl Into<BufSlice>, data: Option<impl Into<BufSlice>>) -> Event {
        Event::ProcessingInstruction {
            name: name.into(),
            data: data.map(Into::into),
        }
    }

    pub fn start_element(name: Name, attributes: impl IntoIterator<Item = Attribute>) -> Event {
        Event::StartElement {
            name,
            attributes: attributes.into_iter().collect(),
        }
    }

    pub fn end_element(name: Name) -> Event {
        Event::EndElement { name }
    }

    pub fn cdata(data: impl Into<BufSlice>) -> Event {
        Event::CData(data.into())
    }

    pub fn comment(data: impl Into<BufSlice>) -> Event {
        Event::Comment(data.into())
    }

    pub fn text(data: impl Into<BufSlice>) -> Event {
        Event::Text(data.into())
    }

    pub fn whitespace(data: impl Into<BufSlice>) -> Event {
        Event::Whitespace(data.into())
    }

    pub fn start_element_name(&self) -> Name {
        match self {
            Event::StartElement { name, .. } => *name,
            _ => panic!("Event is not start element"),
        }
    }

    pub fn attributes(&self) -> &[Attribute] {
        match self {
            Event::StartElement { attributes, .. } => attributes,
            _ => panic!("Event is not start element"),
        }
    }

    pub fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            Event::StartElement { attributes, .. } => attributes,
            _ => panic!("Event is not start element"),
        }
    }

    pub fn end_element_name(&self) -> Name {
        match self {
            Event::EndElement { name, .. } => *name,
            _ => panic!("Event is not start element"),
        }
    }

    pub fn as_text(&self) -> BufSlice {
        match self {
            Event::Text(data) => *data,
            _ => panic!("Event is not text"),
        }
    }

    pub fn as_text_ref_mut(&mut self) -> &mut BufSlice {
        match self {
            Event::Text(data) => data,
            _ => panic!("Event is not text"),
        }
    }

    pub fn as_reified<'buf>(&self, buffer: &'buf Buffer) -> ReifiedEvent<'buf> {
        match *self {
            Event::StartDocument {
                version,
                ref encoding,
                standalone,
            } => ReifiedEvent::start_document(version, encoding.as_reified(buffer), standalone),
            Event::EndDocument => ReifiedEvent::end_document(),
            Event::DoctypeDeclaration { content } => ReifiedEvent::doctype_declaration(content.as_reified(buffer)),
            Event::ProcessingInstruction { name, data } => {
                ReifiedEvent::processing_instruction(name.as_reified(buffer), data.map(|d| d.as_reified(buffer)))
            }
            Event::StartElement { name, ref attributes } => ReifiedEvent::start_element(
                name.as_reified(buffer),
                attributes.iter().cloned().map(|a| a.as_reified(buffer)),
                Namespace::empty(),
            ),
            Event::EndElement { name } => ReifiedEvent::end_element(name.as_reified(buffer)),
            Event::CData(data) => ReifiedEvent::cdata(data.as_reified(buffer)),
            Event::Comment(data) => ReifiedEvent::comment(data.as_reified(buffer)),
            Event::Text(data) => ReifiedEvent::text(data.as_reified(buffer)),
            Event::Whitespace(data) => ReifiedEvent::whitespace(data.as_reified(buffer)),
        }
    }
}

#[derive(Debug, Clone, From)]
pub enum CowEvent {
    Ephemeral(Event),
    Reified(ReifiedEvent<'static>),
}

impl CowEvent {
    pub fn reify_in_place(&mut self, buffer: &Buffer) {
        match self {
            CowEvent::Ephemeral(e) => {
                let reified = e.as_reified(buffer).into_owned();
                *self = CowEvent::Reified(reified);
            }
            CowEvent::Reified(_) => {} // nothing to do
        }
    }

    pub fn reify(self, buffer: &Buffer) -> ReifiedEvent {
        match self {
            CowEvent::Ephemeral(e) => e.as_reified(buffer),
            CowEvent::Reified(e) => e,
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            CowEvent::Ephemeral(Event::Text(_)) => true,
            CowEvent::Reified(ReifiedEvent::Text(_)) => true,
            _ => false,
        }
    }

    pub fn is_end_element(&self) -> bool {
        match self {
            CowEvent::Ephemeral(Event::EndElement { .. }) => true,
            CowEvent::Reified(ReifiedEvent::EndElement { .. }) => true,
            _ => false,
        }
    }
}
