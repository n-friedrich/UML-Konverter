use std::collections::HashMap;

pub enum Reg_Col_Type {
    BASE, //Regexe für START, TITEL, END, COMMENT und EMPTY, enthalten in allen folgenden
    CLASSES, //Regexe für Klassendiagramme
}

#[derive(Eq)]
#[derive(Hash)]
pub enum Regexpiece {
    START,
    TITEL,
    END,
    COMMENT,
    EMPTY,
    PACKAGE,
}

pub fn regex_collection_init(give: Reg_Col_Type) -> HashMap<Regexpiece, &str> {
    let psblComment = r"\s*(#[^\n\r]*\s*)?(\r|\n)+";
    let psblCommentC = r"\s*,\s*(#[^\n\r]*\s*)?(\r|\n)+";
    let psblCommentS = r"\s*{\s*(#[^\n\r]*\s*)?(\r|\n)+";
    let mut map: HashMap<Regexpiece, &str> = HashMap::new();
    map.insert(Regexpiece::START, format!("{}{}", r"\s*@start\w", psblComment));
    map.insert(Regexpiece::TITEL, format!("{}{}", r"\s*title:(?-u:\22)?P<name>\w+(?-u:\22)", psblCommentC));
    map.insert(Regexpiece::END, format!("{}{}", r"\s*@end", psblComment));
    map.insert(Regexpiece::COMMENT, r"\s*#[^\n\r]*\s*(\r|\n)+");
    map.insert(Regexpiece::EMPTY, r"\s*(\r|\n)+");
    match give {
        Reg_Col_Type::BASE => return map,
        Reg_Col_Type::CLASSES => return col_classes(map),
    }
}

fn col_classes(map: HashMap<Regexpiece, &str>) -> HashMap<Regexpiece, &str> {
    let psblComment = r"\s*(#[^\n\r]*\s*)?(\r|\n)+";
    let psblCommentC = r"\s*,\s*(#[^\n\r]*\s*)?(\r|\n)+";
    let psblCommentS = r"\s*{\s*(#[^\n\r]*\s*)?(\r|\n)+";
    map.insert(Regexpiece::PACKAGE, format!("{}{}", r"\s*package:(?-u:\22)?P<name>\w+(?-u:\22)", psblCommentC));
    return map;
}