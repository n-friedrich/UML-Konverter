extern crate regex;

mod structures;
mod parser;
mod test;
//mod regexCollection;

fn main() {
    test::test_klassendiagramm(true);
    /* Beim Entgegennehmen des Diagrams muss gepüft werden, ob Probleme wie
     * NOENDOFSCOPE ode NOSTART bestehen! Falls ja --> Programm mit der Fehlerauflistung abbrechen! */
}
