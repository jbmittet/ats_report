use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Analysis {
    pub file: String,
    pub structure: Structure,
    pub pages: Vec<PageInfo>,
    pub contact: Contact,
    pub sections: Vec<SectionInfo>,
    pub date_ranges: Vec<String>,
    pub skills: SkillsReport,
    pub job_match: Option<JobMatch>,
    pub findings: Vec<Finding>,
    pub score: Score,
    pub full_text: String,
    pub extractor: String,
}

#[derive(Serialize, Default)]
pub struct Structure {
    pub page_count: usize,
    pub encrypted: bool,
    pub image_count: usize,
    pub total_words: usize,
    pub total_chars: usize,
}

#[derive(Serialize)]
pub struct PageInfo {
    pub page: usize,
    pub words: usize,
    pub lines: usize,
    pub short_line_ratio: f32,
    pub weird_glyphs: usize,
    pub images: usize,
    pub status: String, // ok | empty | suspicious
    pub text: String,
}

#[derive(Serialize, Default)]
pub struct Contact {
    pub name_guess: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub urls: Vec<String>,
}

#[derive(Serialize)]
pub struct SectionInfo {
    pub name: String,
    pub found: bool,
    pub line_count: usize,
}

#[derive(Serialize, Default)]
pub struct SkillsReport {
    pub total: usize,
    pub frontend_count: usize,
    pub categories: Vec<SkillCategory>,
    /// flat list of canonical names, handy for JSON consumers
    pub all: Vec<String>,
}

#[derive(Serialize)]
pub struct SkillCategory {
    pub category: String,
    pub skills: Vec<SkillHit>,
}

#[derive(Serialize)]
pub struct SkillHit {
    pub name: String,
    /// which spelling in the resume triggered the match
    pub matched_as: String,
    pub mentions: usize,
}

#[derive(Serialize)]
pub struct JobMatch {
    /// skills the job description asks for (per taxonomy)
    pub required_skills: Vec<String>,
    pub matched: Vec<String>,
    pub missing: Vec<String>,
    pub match_percent: u32,
    /// frequent non-skill words from the JD that are absent from the resume
    pub other_missing_keywords: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Finding {
    pub severity: String, // error | warning | info
    pub message: String,
}

#[derive(Serialize, Default)]
pub struct Score {
    pub total: u32,
    pub grade: String,
    pub breakdown: Vec<ScoreItem>,
}

#[derive(Serialize, Debug)]
pub struct ScoreItem {
    pub category: String,
    pub earned: u32,
    pub max: u32,
    pub reason: String,
}

impl Analysis {
    pub fn error(&mut self, m: impl Into<String>) {
        self.findings.push(Finding {
            severity: "error".into(),
            message: m.into(),
        });
    }
    pub fn warn(&mut self, m: impl Into<String>) {
        self.findings.push(Finding {
            severity: "warning".into(),
            message: m.into(),
        });
    }
    pub fn info(&mut self, m: impl Into<String>) {
        self.findings.push(Finding {
            severity: "info".into(),
            message: m.into(),
        });
    }
}
