from reportlab.lib.pagesizes import A4
from reportlab.pdfgen import canvas
from reportlab.platypus import Table, TableStyle
from reportlab.lib import colors
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent

def low(path):
    # Image-only text, two-column table, no headers, no contact info
    c = canvas.Canvas(path, pagesize=A4)
    t = Table([["Dev", "Worked on stuff"], ["Built things", "Python"]])
    t.setStyle(TableStyle([("GRID", (0,0), (-1,-1), 0, colors.white)]))
    t.wrapOn(c, 400, 400); t.drawOn(c, 50, 600)
    c.drawString(50, 300, "•  ★  ✦  ◆  ▶  ■  ●")  # odd glyphs
    c.save()

def middle(path):
    # Real text, email and headers, but missing skills/dates and a table layout
    c = canvas.Canvas(path, pagesize=A4)
    y = 780
    for line in [
        "John Doe",
        "john.doe@example.com | +1 555 0100",
        "",
        "Experience",
        "Software Engineer, Acme Corp",
        "Built internal tools and maintained services.",
        "",
        "Education",
        "BSc Computer Science, State University",
    ]:
        c.drawString(50, y, line); y -= 18
    c.save()

def high(path):
    # Single column, clear section headers, contact info with links, dates, skills
    c = canvas.Canvas(path, pagesize=A4)
    y = 780
    lines = [
        ("John Doe", 16),
        ("john.doe@example.com | +1 555 0100 | linkedin.com/in/johndoe | https://github.com/johndoedev", 9),
        ("", 9),
        ("EXPERIENCE", 12),
        ("Senior Software Engineer, Acme Corp (2021 - Present)", 10),
        ("- Led migration of 12 services to Kubernetes, reducing deploy time by 60%.", 10),
        ("- Mentored 4 engineers; introduced code review standards.", 10),
        ("", 9),
        ("Software Engineer, Beta Inc (2018 - 2021)", 10),
        ("- Built REST APIs in Python and Go serving 2M requests/day.", 10),
        ("", 9),
        ("EDUCATION", 12),
        ("BSc Computer Science, State University, 2018", 10),
        ("", 9),
        ("SKILLS", 12),
        ("Python, Go, Rust, Kubernetes, PostgreSQL, AWS, CI/CD", 10),
    ]
    for text, size in lines:
        c.setFont("Helvetica", size)
        c.drawString(50, y, text); y -= size + 6
    c.save()

low(str(PROJECT_ROOT / "low_score.pdf"))
middle(str(PROJECT_ROOT / "middle_score.pdf"))
high(str(PROJECT_ROOT / "high_score.pdf"))