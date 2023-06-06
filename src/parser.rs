use std::fs;

pub struct Section {
    title: String,
    content: String,
    subsections: Vec<Section>,
}

pub struct Post {
    sections: Vec<Section>,
    title: String,
    description: String, 
}


impl Post {
    pub fn build_html(&self) -> String {
        let mut html: String = format!("<h1>{}</h1>\n <p>{}</p>", &self.title, &self.description);

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
        let mut descrip: String;
        let mut titl: String;
        let mut i = 0;
        while i < count{
            let start = lines[i].split("!").collect::<Vec<&str>>();
            if start[0].contains("#") && start[0].len() > depth {
                
            } else {
                titl = start[1].to_owned();
                i+=1;
                descrip = lines[i].to_owned();
            }
            i+=1;
        }

        Post {
            title : titl,
            description : descrip,
            sections : Vec::new(),
        }
    }
    
}

impl Section {
    pub fn build_html(&self, depth: i32) -> String {
        let mut html: String = format!("<h{}>{}</h{}>", depth, &self.title, depth);
        let content: &str = &format!("<p>{}</p>", &self.content);
        html.push_str(content);

        for subsection in &self.subsections {
            let html_subsection: &str = &subsection.build_html(depth + 1);
            html.push_str(html_subsection)
        }

        html
    }

    //pub fn from_vec(Vec<&str>)
}