#[derive(Debug)]
pub enum Diagramtype {
    ACTIVITY,
    USECASE,
    CLASSES,
    SEQUENCE,
    NONE,
}

pub enum Problem {
    NONE,
    WRONGLINE(i32),
}

pub struct Connection {
    node1: String, //Nodename 1
    node2: String, //Nodename 2
    description: String, //Anmerkungen am Pfeil
    contype: String, //Verbindungstyp wie in Diagramm evtl ersetzen mit enum
}

pub struct Node {
    //Fuer Klassen und Anwendungen
    nodetype: String, //Nodetyp (Name) evtl erstetzen mit enum
    name: String, //Name des Nodes
    stereotype: String, //Stereotyp des Nodes
    variables: Vec<String>, //Liste mit Variablen
    methods: Vec<String>, //Liste mit Methoden
}

pub struct Package {
    name: String, //Name des Packages
    nodes: Vec<Node>, //Liste mit allen Nodes im Package
    connections: Vec<Connection>, //Liste mit allen Connections innerhalb des Packages
}

pub struct Participant {
    name: String, //Name des Participanten
    comment: String, //Kommentar zum Participanten
}

pub struct Note {
    side: String, //Seite der Notitz evtl ersetzen durch enum
    content: String, //Inhalt der Notitz
}

pub struct Activity {
    name: String, //Name der Aktivitaet
    notes: Vec<Note>, //Liste mit Notitzen an der Aktivitaet
}

pub struct Result {
    description: String, //Beschreibung des Ergebnisses (z.B. ja/nein)
    activities: Vec<Activity>, //Liste mit Aktivitaeten in dem Ergebnis
}

pub struct Condition {
    name: String, //Name der Bedingung
    results: Vec<Result>, //Liste mit allen Ergebnissen
}

pub struct Loop {
    name: String, //Beschreibung/Bedingung der Schleife
    doWhile: bool, //Gesetzt wenn Schleife do-while
    activities: Vec<Activity>, //Liste mit Aktivitaeten in der Schleife
}

pub struct Parallel {
    activities: Vec<Activity>, //Liste mit Aktivitaeten, die parallel ablaufen sollen
}

pub struct Diagram {
    diagramtype: String, //Diagrammtyp wie in Diagramm evtl ersetzen mit enum
    name: String, //Diagrammname
    packages: Vec<Package>, //Liste mit allen Packages
    nodes: Vec<Node>, //Liste mit allen Nodes außerhalb von Packages
    connections: Vec<Connection>, //Liste mit allen Verbindungen außerhalb von Packages
}