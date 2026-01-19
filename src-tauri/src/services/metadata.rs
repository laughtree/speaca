use serde::{Deserialize, Serialize};

use crate::services::versioning::{new_uid, save_version};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct chapterHeader {
    pub parent_work: String,
    pub chapter_number: u32,
    pub title: String,
    pub uid: String,
    pub tags: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct chapter {
    pub header: chapterHeader,
    pub body: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct chapterReference {
    pub title: String,
    pub uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct workInfo {
    pub title: String,
    pub uid: String,
    pub chapters: Vec<Vec<chapterReference>>,
    pub author: String,
    pub description: String,
    pub tags: Vec<String>,
    pub latest_edited_chapter: u32,
    pub latest_edited_branch: u32,
}

impl workInfo {
    pub fn new(title: String, author: String) -> Self {
        let mut chapters: Vec<Vec<chapterReference>> = Vec::new();
        chapters.push(Vec::new());
        chapters[0].push(chapterReference {
            title: "Chapter 1".to_string(),
            uid: new_uid(),
        });

        workInfo {
            title,
            uid: new_uid(),
            chapters,
            author,
            description: String::new(),
            tags: Vec::new(),
            latest_edited_chapter: 0,
            latest_edited_branch: 0,
        }
    }
    pub fn update_latest(&mut self, chapter: u32, branch: u32) {
        self.latest_edited_chapter = chapter;
        self.latest_edited_branch = branch;
    }
}

impl chapterHeader {
    pub fn new(title: String, parent_work: String, chapter_number: u32, uid: String) -> Self {
        chapterHeader {
            parent_work,
            chapter_number,
            title,
            uid,
            tags: Vec::new(),
        }
    }
}

impl chapter {
    pub fn new (parent_work: &workInfo, chapInfo: &chapterReference) -> Self {
        let work = parent_work.clone();
        let chap = chapInfo.clone();
        chapter {
            header: chapterHeader::new(chap.title, work.title, work.latest_edited_chapter, chap.uid),
            body: serde_json::from_str("{}").expect("")
        }
    }
}