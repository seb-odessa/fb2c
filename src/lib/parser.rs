use std::io::Read;
use iconv::IconvDecodable;

const CHUNK: usize = 128;


pub fn load_header<F: Read>(file: &mut F) -> Option<String> {
    let mut buffer = [0u8; CHUNK];

    const CLOSE_DS_TAG: &[u8] = "</description>".as_bytes();
    const CLOSE_FB_TAG: &[u8] = "</FictionBook>".as_bytes();
    const BYTE_ORDER_MARK: [u8; 3] = [0xEF, 0xBB, 0xBF];

    let stream = file.by_ref();
    let mut header: Vec<u8> = Vec::new();
    while let Some(read) = stream.take(CHUNK as u64).read(&mut buffer).ok() {
        if 0 == read {
            break;
        }

        let mut xml_pos = 0;
        if header.is_empty() {
            // Let's skip BOM if exists
            if let Some(pos) = find(&buffer[0..read], &BYTE_ORDER_MARK) {
                if 0 == pos {
                    xml_pos = BYTE_ORDER_MARK.len();
                }
            }
        }

        header.extend_from_slice(&buffer[xml_pos..read]);
        let lookup_window_pos = if header.len() > CHUNK + CLOSE_DS_TAG.len() {
            header.len() - CHUNK - CLOSE_DS_TAG.len()
        } else {
            0
        };

        if let Some(pos) = find(&header[lookup_window_pos..], CLOSE_DS_TAG) {
            header.resize(lookup_window_pos + pos, 0u8);
            header.extend_from_slice(CLOSE_DS_TAG);
            header.extend_from_slice(CLOSE_FB_TAG);
            return convert_utf8(header)
                   .and_then(|s| Some(escape_non_xml_chars(s)))
        }
    }
    return None;
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn find_bounds(header: &[u8], beg: &str, end: &str) -> Option<(usize, usize)> {
    if let Some(pos) = find(header, beg.as_bytes()) {
        let spos = pos + beg.len();
        if let Some(mut epos) = find(&header[spos..], end.as_bytes()) {
            epos = epos + spos;
            //println!("=> [{}]", String::from_utf8_lossy(&header[spos..epos]).to_string());
            return Some((spos, epos));
        }
    }
    None
}

fn get_encoding(header: &[u8]) -> Option<&str> {
    if let Some((s_decl, e_decl)) = find_bounds(header, "<?xml ", "?>") {
        let encoding =
            if let Some((s_enc, e_enc)) = find_bounds(&header[s_decl..e_decl], "encoding=\"", "\"") {
                String::from_utf8_lossy(&header[s_decl+s_enc..s_decl+e_enc]).to_string()
            } else if let Some((s_enc, e_enc)) = find_bounds(&header[s_decl..e_decl], "encoding='", "'") {
                String::from_utf8_lossy(&header[s_decl+s_enc..s_decl+e_enc]).to_string()
            } else {
                String::default()
            };
        match encoding.to_lowercase().as_str() {
            "utf-8" => Some("utf-8"),
            "koi8-r" => Some("koi8-r"),
            "windows-1251" => Some("cp1251"),
            _ => None,
        }

    } else {
        None
    }
}

fn convert_utf8(header: Vec<u8>) -> Option<String> {
    let end = std::cmp::min(header.len(), CHUNK);
    if let Some(encoding) = get_encoding(&header[0..end]) {
        if encoding != "utf-8" {
            return header.decode_with_encoding(encoding);
        }
    }
    Some(String::from_utf8_lossy(&header).to_string())
}

fn escape_non_xml_chars(xml: String) -> String {
    xml.replace("&","&amp;")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_header() {
        //
        let data = "123456789012345678901234567890123456789012345678901234567890</description><body>..................................................................</body>".as_bytes();
        //ABCD
        let mut stream = data.clone();

        let readed = load_header(&mut stream);

        println!(">{}", String::from_utf8_lossy(data));
        println!(">{}", readed.unwrap_or_default());

        let _dd = r##"
        <?xml version="1.0" encoding="utf-8"?>
        <FictionBook xmlns:l="http://www.w3.org/1999/xlink" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.gribuser.ru/xml/fictionbook/2.0">
            <description>
                <title-info>
                    <genre>religion_self</genre>
                    <author><first-name>Александр</first-name><middle-name>Николаевич</middle-name><last-name>Медведев</last-name></author>
                    <author><first-name>Ирина</first-name><middle-name>Борисовна</middle-name><last-name>Медведева</last-name></author>
                    <book-title>Игра под названием Жизнь</book-title>
                    <lang>ru</lang>
                    <sequence number="3" name="Технология счастья"/>
                    <annotation>
                        <p>Искусство быть счастливым, побеждать обстоятельства и понимать себя и других...</p>
                        <empty-line/>
                        <p>Александр и Ирина Медведевы - широко известные в России и за рубежом авторы. Александр Медведев, которого на Западе называют "русским Кастанедой", стал первым европейцем, посвященным в тайные знания древнего даосского клана Шоу-Дао - "Спокойных" или "Бессмертных". История его ученичества описана в четырех томах серии "Путь Шоу-Дао", выдержавших многократные переиздания. С момента своего появления на свет люди автоматически вовлекаются в сложнейшую и в то же время увлекательнейшую игру, которая называется "жизнь". Правила и технику этой игры им приходится постигать методом проб и ошибок, проходя через боль и разочарования. К сожалению, далеко не всем удается стать искусными игроками.</p>
                        <p>Клан Шоу-Дао в течение нескольких тысячелетий развивал и совершенствовал "Искусство Жизни". Эта книга поможет понять некоторые важные правила "игры под названием жизнь", стать опытным и удачливым игроком, умеющим достигать поставленных целей, быть счастливым и успешным в работе и в личной жизни.</p>
                    </annotation>
                    <document-info>
                    </document-info>
                </title-info>
                <publish-info><isbn>5-17-038620-6</isbn><isbn>5-9713-2994-4</isbn></publish-info>
            </description>
            </FictionBook>
        "##;
    }
}
