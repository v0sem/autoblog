use std::fs;

#[derive(Debug)]
pub struct Section {
    title: String,
    content: String,
    subsections: Vec<Section>,
}

#[derive(Debug)]
pub struct Post {
    sections: Vec<Section>,
    title: String,
    content: String,
}

impl Post {
    pub fn build_html(&self) -> String {
        let mut html: String = format!("<h1>{}</h1>\n<p>{}</p>\n", &self.title, &self.content);

        for section in &self.sections {
            let html_section: &str = &section.build_html(2);
            html.push_str(html_section);
        }

        html
    }

    pub fn from_file(path: &str) -> Post {
        let content = fs::read_to_string(format!("static/blog/{}", path))
            .expect("File should exist and be readable");

        let depth = 1;
        let lines: Vec<&str> = content.lines().collect::<Vec<&str>>();
        let count = lines.len();
        let mut descrip: String = String::from("");
        let mut titl: String = String::from("");
        let mut i = 0;
        let mut sec_list: Vec<Section> = Vec::new();

        while i < count {
            let start = lines[i].split("!").collect::<Vec<&str>>();

            if start.len() > 1 && start[1].contains("#") && start[1].len() > depth {
                let sec = Section::from_vec(&lines, i, depth+1);
                i += sec.len();
                sec_list.push(sec);
            } else {
                titl = start[2].to_owned();
                descrip = lines[i + 1].to_owned();
                i += 2;
            }
        }

        Post {
            title: titl,
            content: descrip,
            sections: sec_list,
        }
    }
}

impl Section {
    pub fn build_html(&self, depth: i32) -> String {
        let mut html: String = format!("<h{}>{}</h{}>\n", depth, &self.title, depth);
        let content: &str = &format!("<p>{}</p>\n", &self.content);
        html.push_str(content);

        for subsection in &self.subsections {
            let html_subsection: &str = &subsection.build_html(depth + 1);
            html.push_str(html_subsection)
        }

        html
    }

    pub fn from_vec(lines:&Vec<&str>, section_start:usize, depth: usize) -> Section {
        let mut i = section_start + 2;
        if i >= lines.len() {
            return Section {
                title: lines[section_start].split("!").collect::<Vec<&str>>()[2].to_owned(),
                content: lines[section_start+1].to_owned(),
                subsections: Vec::new(),
            };
        }
        let mut sec_depth = lines[i].split("!").collect::<Vec<&str>>()[1].len();
        let mut ssec: Vec<Section> = Vec::new();

        while sec_depth > depth {
            ssec.push(Section::from_vec(&lines, i, sec_depth));
            i += 2;

            if i >= lines.len() {
                break;
            }
            else {
                sec_depth = lines[i].split("!").collect::<Vec<&str>>()[1].len();
            }
        }

        Section {
            title: lines[section_start].split("!").collect::<Vec<&str>>()[2].to_owned(),
            content: lines[section_start+1].to_owned(),
            subsections: ssec,
        }
    }

    pub fn len(&self) -> usize {
        let mut i = 2;
        for sec in &self.subsections {
            i += sec.len();
        }
        i
    }
}
