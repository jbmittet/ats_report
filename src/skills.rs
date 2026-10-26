use crate::models::{SkillCategory, SkillHit, SkillsReport};

pub type SkillDef = (
    &'static str,
    &'static [(&'static str, &'static [&'static str])],
);

pub const SKILL_TAXONOMY: &[SkillDef] = &[
    (
        "Frontend: Languages & Markup",
        &[
            ("HTML", &["html5", "html"]),
            ("CSS", &["css3", "css"]),
            (
                "JavaScript",
                &["javascript", "js", "es6", "es2015", "ecmascript"],
            ),
            ("TypeScript", &["typescript", "ts"]),
        ],
    ),
    (
        "Frontend: Frameworks & Libraries",
        &[
            ("React", &["react", "reactjs", "react.js"]),
            ("Next.js", &["next.js", "nextjs", "next js"]),
            ("Vue", &["vue", "vuejs", "vue.js", "vue 3"]),
            ("Nuxt", &["nuxt", "nuxt.js", "nuxtjs"]),
            ("Angular", &["angular", "angularjs", "angular.js"]),
            ("Svelte", &["svelte", "sveltekit"]),
            ("Solid", &["solidjs", "solid.js"]),
            ("Remix", &["remix"]),
            ("Gatsby", &["gatsby", "gatsbyjs"]),
            ("jQuery", &["jquery"]),
            ("Backbone", &["backbone.js", "backbonejs"]),
            ("Ember", &["ember.js", "emberjs"]),
            ("Astro", &["astro"]),
            ("Preact", &["preact"]),
            ("Lit", &["lit-element", "lit html"]),
        ],
    ),
    (
        "Frontend: Styling & UI",
        &[
            ("Sass/SCSS", &["sass", "scss"]),
            ("Less", &["less.js"]),
            ("Tailwind CSS", &["tailwind", "tailwindcss", "tailwind css"]),
            ("Bootstrap", &["bootstrap"]),
            ("Material UI", &["material ui", "material-ui", "mui"]),
            ("Chakra UI", &["chakra ui", "chakra-ui"]),
            ("Ant Design", &["ant design", "antd"]),
            (
                "Styled Components",
                &["styled-components", "styled components", "emotion"],
            ),
            ("CSS Modules", &["css modules", "css-modules"]),
            (
                "shadcn/ui",
                &["shadcn", "shadcn/ui", "radix ui", "radix-ui"],
            ),
            (
                "Responsive Design",
                &[
                    "responsive design",
                    "responsive web design",
                    "mobile-first",
                    "mobile first",
                ],
            ),
            ("Flexbox/Grid", &["flexbox", "css grid"]),
            ("Figma", &["figma"]),
            ("Storybook", &["storybook"]),
            (
                "Design Systems",
                &[
                    "design system",
                    "design systems",
                    "component library",
                    "component libraries",
                ],
            ),
        ],
    ),
    (
        "Frontend: State & Data Fetching",
        &[
            ("Redux", &["redux", "redux toolkit", "rtk"]),
            ("Zustand", &["zustand"]),
            ("MobX", &["mobx"]),
            ("Recoil", &["recoil"]),
            ("Pinia/Vuex", &["pinia", "vuex"]),
            ("NgRx/RxJS", &["ngrx", "rxjs"]),
            (
                "React Query",
                &["react query", "tanstack query", "react-query", "swr"],
            ),
            ("Apollo", &["apollo client", "apollo"]),
            ("GraphQL", &["graphql"]),
            ("REST APIs", &["rest", "restful", "rest api", "rest apis"]),
            ("WebSockets", &["websocket", "websockets", "socket.io"]),
        ],
    ),
    (
        "Frontend: Tooling & Build",
        &[
            ("Webpack", &["webpack"]),
            ("Vite", &["vite"]),
            ("Babel", &["babel"]),
            ("Rollup", &["rollup"]),
            ("esbuild", &["esbuild"]),
            ("Turbopack", &["turbopack"]),
            ("npm/yarn/pnpm", &["npm", "yarn", "pnpm"]),
            ("ESLint/Prettier", &["eslint", "prettier"]),
            ("Monorepo", &["monorepo", "turborepo", "nx", "lerna"]),
            (
                "Micro-frontends",
                &[
                    "micro-frontend",
                    "micro-frontends",
                    "microfrontend",
                    "microfrontends",
                    "module federation",
                ],
            ),
        ],
    ),
    (
        "Frontend: Testing",
        &[
            ("Jest", &["jest"]),
            ("Vitest", &["vitest"]),
            (
                "React Testing Library",
                &["react testing library", "testing library", "rtl"],
            ),
            ("Cypress", &["cypress"]),
            ("Playwright", &["playwright"]),
            ("Selenium", &["selenium"]),
            ("Mocha/Chai", &["mocha", "chai"]),
            ("Enzyme", &["enzyme"]),
        ],
    ),
    (
        "Frontend: Web Platform & Quality",
        &[
            (
                "Accessibility",
                &["accessibility", "a11y", "wcag", "aria", "section 508"],
            ),
            (
                "Performance",
                &[
                    "web performance",
                    "core web vitals",
                    "lighthouse",
                    "lazy loading",
                    "code splitting",
                ],
            ),
            ("SEO", &["seo", "search engine optimization"]),
            (
                "SSR/SSG",
                &[
                    "ssr",
                    "ssg",
                    "server-side rendering",
                    "server side rendering",
                    "static site generation",
                    "isr",
                ],
            ),
            (
                "PWA",
                &[
                    "pwa",
                    "progressive web app",
                    "progressive web apps",
                    "service worker",
                    "service workers",
                ],
            ),
            (
                "Web Components",
                &["web components", "shadow dom", "custom elements"],
            ),
            (
                "Cross-browser",
                &["cross-browser", "cross browser", "browser compatibility"],
            ),
            (
                "i18n/l10n",
                &["i18n", "l10n", "internationalization", "localization"],
            ),
            (
                "Three.js/WebGL",
                &["three.js", "threejs", "webgl", "canvas api", "d3.js", "d3"],
            ),
            (
                "Animation",
                &["framer motion", "gsap", "css animations", "lottie"],
            ),
            ("React Native", &["react native"]),
            ("Electron", &["electron"]),
        ],
    ),
    (
        "Backend & Languages",
        &[
            ("Node.js", &["node.js", "nodejs", "node js", "node"]),
            ("Express", &["express.js", "expressjs", "express"]),
            ("NestJS", &["nestjs", "nest.js"]),
            ("Python", &["python"]),
            ("Django", &["django"]),
            ("Flask", &["flask"]),
            ("FastAPI", &["fastapi"]),
            ("Java", &["java"]),
            ("Spring", &["spring boot", "spring"]),
            ("Kotlin", &["kotlin"]),
            ("Go", &["golang", "go lang"]),
            ("Rust", &["rust"]),
            ("C#/.NET", &["c#", ".net", "dotnet", "asp.net"]),
            ("C/C++", &["c++", "cpp"]),
            ("PHP", &["php", "laravel"]),
            ("Ruby", &["ruby", "ruby on rails", "rails"]),
            ("Swift", &["swift"]),
        ],
    ),
    (
        "Data & Storage",
        &[
            ("SQL", &["sql"]),
            ("PostgreSQL", &["postgresql", "postgres"]),
            ("MySQL", &["mysql"]),
            ("MongoDB", &["mongodb", "mongo"]),
            ("Redis", &["redis"]),
            ("Firebase", &["firebase", "firestore"]),
            ("Elasticsearch", &["elasticsearch", "opensearch"]),
            ("Kafka", &["kafka"]),
            ("Spark", &["spark"]),
        ],
    ),
    (
        "Cloud & DevOps",
        &[
            ("AWS", &["aws", "amazon web services"]),
            ("Azure", &["azure"]),
            ("GCP", &["gcp", "google cloud"]),
            ("Docker", &["docker"]),
            ("Kubernetes", &["kubernetes", "k8s"]),
            ("Terraform", &["terraform"]),
            (
                "CI/CD",
                &[
                    "ci/cd",
                    "ci cd",
                    "continuous integration",
                    "github actions",
                    "gitlab ci",
                    "jenkins",
                    "circleci",
                ],
            ),
            ("Git", &["git", "github", "gitlab", "bitbucket"]),
            ("Vercel/Netlify", &["vercel", "netlify", "cloudflare"]),
            ("Linux", &["linux", "unix"]),
            ("Microservices", &["microservices", "microservice"]),
        ],
    ),
    (
        "AI & Data Science",
        &[
            ("Machine Learning", &["machine learning", "ml"]),
            ("Deep Learning", &["deep learning"]),
            ("TensorFlow", &["tensorflow"]),
            ("PyTorch", &["pytorch"]),
            ("NLP", &["nlp", "natural language processing"]),
            ("LLMs", &["llm", "llms", "openai", "langchain"]),
        ],
    ),
    (
        "Process & Soft Skills",
        &[
            ("Agile/Scrum", &["agile", "scrum", "kanban"]),
            ("Jira", &["jira", "confluence"]),
            ("TDD", &["tdd", "test-driven", "test driven"]),
            (
                "Code Review",
                &[
                    "code review",
                    "code reviews",
                    "pull request",
                    "pull requests",
                ],
            ),
            (
                "Leadership",
                &[
                    "leadership",
                    "mentoring",
                    "mentorship",
                    "team lead",
                    "tech lead",
                ],
            ),
            (
                "Communication",
                &["communication", "collaboration", "stakeholder"],
            ),
            (
                "Project Management",
                &["project management", "product management"],
            ),
            (
                "UX/UI",
                &[
                    "ux",
                    "ui/ux",
                    "user experience",
                    "user interface",
                    "usability",
                ],
            ),
        ],
    ),
];

/// Terms that are common English words / fragments. They only count on
/// "list-like" lines (comma/pipe/bullet separated), typical of skills sections.
pub const AMBIGUOUS: &[&str] = &[
    "go", "rest", "less.js", "swift", "spark", "node", "express", "remix", "astro", "lit html",
    "solid", "ml", "nx", "rtl", "ts", "js", "swr", "mui", "git", "rails", "spring", "enzyme",
    "babel", "rollup", "ember", "ux", "aria", "d3",
];

/// Count whole-term occurrences of `term` in `haystack` (both lowercase).
/// A boundary is any char that can't be part of a skill token.
pub fn count_term(haystack: &str, term: &str) -> usize {
    if term.is_empty() {
        return 0;
    }
    let is_tok = |c: char| c.is_ascii_alphanumeric() || matches!(c, '+' | '#');
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = haystack[start..].find(term) {
        let s = start + pos;
        let e = s + term.len();

        let before_ok = haystack[..s].chars().next_back().is_none_or(|c| !is_tok(c));
        let after_ok = haystack[e..].chars().next().is_none_or(|c| !is_tok(c));

        if before_ok && after_ok {
            count += 1;
        }
        start = e;
    }
    count
}

fn is_listlike(line: &str) -> bool {
    line.matches(',').count() >= 2
        || line.contains('|')
        || line.contains('•')
        || line.contains('·')
        || line.contains('/')
        || line.contains(';')
}

pub fn detect_skills(text: &str) -> SkillsReport {
    let lower = text.to_lowercase();
    // Pre-compute list-like lines once, for the ambiguous-term check.
    let listlike: String = lower
        .lines()
        .filter(|l| is_listlike(l))
        .collect::<Vec<_>>()
        .join("\n");

    let mut report = SkillsReport::default();

    for (category, skills) in SKILL_TAXONOMY {
        let mut hits = vec![];

        for (canonical, aliases) in *skills {
            let mut mentions = 0;
            let mut matched_as = String::new();

            for alias in *aliases {
                let n = if AMBIGUOUS.contains(alias) {
                    count_term(&listlike, alias)
                } else {
                    count_term(&lower, alias)
                };
                if n > 0 {
                    if matched_as.is_empty() {
                        matched_as = alias.to_string();
                    }
                    mentions += n;
                }
            }

            if mentions > 0 {
                hits.push(SkillHit {
                    name: canonical.to_string(),
                    matched_as,
                    mentions,
                });
            }
        }

        if !hits.is_empty() {
            if category.starts_with("Frontend") {
                report.frontend_count += hits.len();
            }
            report.total += hits.len();
            report.all.extend(hits.iter().map(|h| h.name.clone()));
            report.categories.push(SkillCategory {
                category: category.to_string(),
                skills: hits,
            });
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has(report: &SkillsReport, name: &str) -> bool {
        report.all.iter().any(|s| s == name)
    }

    fn hit<'a>(report: &'a SkillsReport, name: &str) -> &'a SkillHit {
        report
            .categories
            .iter()
            .flat_map(|c| c.skills.iter())
            .find(|h| h.name == name)
            .unwrap_or_else(|| panic!("skill {name} missing"))
    }

    #[test]
    fn count_term_respects_token_boundaries() {
        assert_eq!(count_term("rust and rust-lang", "rust"), 2);
        assert_eq!(count_term("trust the rust", "rust"), 1);
        assert_eq!(count_term("rust.", "rust"), 1);
        assert_eq!(count_term("aRust", "rust"), 0);
        assert_eq!(count_term("rusty", "rust"), 0);
        assert_eq!(count_term("x rust y rust", "rust"), 2);
    }

    #[test]
    fn count_term_handles_symbols_and_empty_input() {
        assert_eq!(count_term("c++ and c#", "c++"), 1);
        assert_eq!(count_term("c++", "c"), 0); // '+' keeps the 'c' token going
        assert_eq!(count_term("", "rust"), 0);
        assert_eq!(count_term("rust", ""), 0);
    }

    #[test]
    fn detects_skills_and_their_categories() {
        let r = detect_skills("Skills: React, TypeScript, CSS, HTML, JavaScript");
        assert_eq!(r.total, 5);
        assert_eq!(r.frontend_count, 5);
        assert!(has(&r, "React"));
        assert!(has(&r, "TypeScript"));
        assert!(has(&r, "CSS"));
        assert!(has(&r, "HTML"));
        assert!(has(&r, "JavaScript"));

        let fe: Vec<&str> = r
            .categories
            .iter()
            .filter(|c| c.category.starts_with("Frontend"))
            .map(|c| c.category.as_str())
            .collect();
        assert_eq!(
            fe,
            vec![
                "Frontend: Languages & Markup",
                "Frontend: Frameworks & Libraries"
            ]
        );
    }

    #[test]
    fn records_the_alias_that_matched_and_the_mention_count() {
        let r = detect_skills("We use ReactJS and reactjs again");
        let h = hit(&r, "React");
        assert_eq!(h.matched_as, "reactjs");
        assert_eq!(h.mentions, 2);
    }

    #[test]
    fn ambiguous_words_only_count_on_list_like_lines() {
        // "git" and "express" are common English words: prose must not trigger them.
        let prose = detect_skills("I use git every day and I express my ideas in code");
        assert!(!has(&prose, "Git"));
        assert!(!has(&prose, "Express"));

        let list = detect_skills("Tools: Git, Docker, Express, Rust, Python");
        assert!(has(&list, "Git"));
        assert!(has(&list, "Express"));
        assert!(has(&list, "Rust"));
        assert!(has(&list, "Python"));
    }

    #[test]
    fn matches_language_names_case_insensitively() {
        let r = detect_skills("RUST and Postgresql and KUBERNETES");
        assert!(has(&r, "Rust"));
        assert!(has(&r, "PostgreSQL"));
        assert!(has(&r, "Kubernetes"));
    }

    #[test]
    fn unknown_text_yields_an_empty_report() {
        let r = detect_skills("I enjoy hiking and cooking on weekends.");
        assert_eq!(r.total, 0);
        assert_eq!(r.frontend_count, 0);
        assert!(r.categories.is_empty());
    }
}
