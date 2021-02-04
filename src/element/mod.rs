use std::collections::HashMap;

pub struct Element
{
    namespaceURI : String,
    prefix : String,
    localName : String,
    tagName : String,

    id : String,
    className : String,
    classList : Vec<String>,
    slot : String,

    attributes : HashMap<String,String>,
  
  DOMString? getAttribute(DOMString qualifiedName);
  DOMString? getAttributeNS(DOMString? namespace, DOMString localName);
  [CEReactions] void setAttribute(DOMString qualifiedName, DOMString value);
  [CEReactions] void setAttributeNS(DOMString? namespace, DOMString qualifiedName, DOMString value);
  [CEReactions] void removeAttribute(DOMString qualifiedName);
  [CEReactions] void removeAttributeNS(DOMString? namespace, DOMString localName);
  [CEReactions] boolean toggleAttribute(DOMString qualifiedName, optional boolean force);
  boolean hasAttribute(DOMString qualifiedName);
  boolean hasAttributeNS(DOMString? namespace, DOMString localName);

  Attr? getAttributeNode(DOMString qualifiedName);
  Attr? getAttributeNodeNS(DOMString? namespace, DOMString localName);
  [CEReactions] Attr? setAttributeNode(Attr attr);
  [CEReactions] Attr? setAttributeNodeNS(Attr attr);
  [CEReactions] Attr removeAttributeNode(Attr attr);

  ShadowRoot attachShadow(ShadowRootInit init);
  readonly attribute ShadowRoot? shadowRoot;

  Element? closest(DOMString selectors);
  boolean matches(DOMString selectors);
  boolean webkitMatchesSelector(DOMString selectors); // historical alias of .matches

  HTMLCollection getElementsByTagName(DOMString qualifiedName);
  HTMLCollection getElementsByTagNameNS(DOMString? namespace, DOMString localName);
  HTMLCollection getElementsByClassName(DOMString classNames);

  [CEReactions] Element? insertAdjacentElement(DOMString where, Element element); // historical
  void insertAdjacentText(DOMString where, DOMString data); // historical

}

impl Element
{
    pub fn hasAttributes(&self) -> bool
    {
        return self.attributes.len() > 0
    }

    pub fn getAttributeNames(&mut self) -> Keys<String>
    {
        return self.attributes.keys()
    }
}