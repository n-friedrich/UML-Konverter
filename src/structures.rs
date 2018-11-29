#[derive(Debug,Clone,Copy)]
pub enum Diagramtype {
    ACTIVITY,
    USECASE,
    CLASSES,
    SEQUENCE,
    NONE,
}

#[derive(Debug,Clone,Copy)]
#[derive(PartialEq)]
pub enum Problem {
    NONE,
    NOFILE,
    NOTYPE,
    NOSTART,
    WRONGLINE(usize),
    MISSINGARGUMENTS(usize),
    NOENDOFSCOPE(usize),
    UNKNOWN(usize),
}

#[derive(Debug,Clone,Copy)]
pub enum Nodetype {
    CLASS,
    ENUM,
    INTERFACE,
    ABSTRACT,
    ANNOTATION,
    UNKNOWN(usize),
}

#[derive(Debug,Clone,Copy)]
#[derive(PartialEq)]
pub enum Conntype {
    VERERBUNG,
    INTERFACE,
    GESTRICHELT,
    BEINHALTET,
    KOMPOSITION,
    AGGREGATION,
}




//zum zeichnen der connections
#[derive(Clone)]
pub struct Connpoints{
    pub start_x:u32,
    pub start_y:u32,
    pub end_x:u32,
    pub end_y:u32,
    pub connection:Connection,
}

#[derive(Clone)]
pub struct Point{
    pub x:u32,
    pub y:u32
}
#[derive(Clone)]
pub struct Connection {
    pub node1: String, //Nodename 1
    pub node2: String, //Nodename 2
    pub description: String, //Anmerkungen am Pfeil
    pub contype: Conntype, //Verbindungstyp wie in Diagramm
}

#[derive(Clone)]
pub struct Node {
    //Fuer Klassen und Anwendungen
    pub nodetype: Nodetype, //Nodetyp
    pub name: String, //Name des Nodes
    pub stereotype: String, //Stereotyp des Nodes
    pub variables: Vec<String>, //Liste mit Variablen
    pub methods: Vec<String>, //Liste mit Methoden
}

#[derive(Clone)]
pub struct Package {
    pub name: String, //Name des Packages
    pub nodes: Vec<Node>, //Liste mit allen Nodes im Package
    pub connections: Vec<Connection>, //Liste mit allen Connections innerhalb des Packages
}

pub struct Participant {
    pub name: String, //Name des Participanten
    pub comment: String, //Kommentar zum Participanten
}

pub struct Note {
    pub side: String, //Seite der Notitz evtl ersetzen durch enum
    pub content: String, //Inhalt der Notitz
}

pub struct Activity {
    pub name: String, //Name der Aktivitaet
    pub notes: Vec<Note>, //Liste mit Notitzen an der Aktivitaet
}

pub struct Result {
    description: String, //Beschreibung des Ergebnisses (z.B. ja/nein)
    activities: Vec<Activity>, //Liste mit Aktivitaeten in dem Ergebnis
}

pub struct Condition {
    pub name: String, //Name der Bedingung
    pub results: Vec<Result>, //Liste mit allen Ergebnissen
}

pub struct Loop {
    pub name: String, //Beschreibung/Bedingung der Schleife
    pub doWhile: bool, //Gesetzt wenn Schleife do-while
    pub activities: Vec<Activity>, //Liste mit Aktivitaeten in der Schleife
}

pub struct Parallel {
    pub activities: Vec<Activity>, //Liste mit Aktivitaeten, die parallel ablaufen sollen
}
#[derive(Clone)]
pub struct Diagram {
    pub problems: Vec<Problem>, //Liste mit aufgetretenden Problemen
    pub name: String, //Diagrammname
    pub packages: Vec<Package>, //Liste mit allen Packages
    pub nodes: Vec<Node>, //Liste mit allen Nodes außerhalb von Packages
    pub connections: Vec<Connection>, //Liste mit allen Verbindungen außerhalb von Packages
}