use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// ── Frontmatter schema ────────────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
struct StoryFrontmatter {
    title: String,
    slug: String,
}

// ── Markdown helpers ──────────────────────────────────────────────────────────

fn md_to_html(md: &str) -> String {
    let opts = Options::ENABLE_STRIKETHROUGH | Options::ENABLE_SMART_PUNCTUATION;
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

// ── Story loading ─────────────────────────────────────────────────────────────

struct Story {
    title: String,
    slug: String,
    html_content: String,
}

fn load_stories(dir: &str) -> Vec<Story> {
    let matter = Matter::<YAML>::new();

    let mut paths: Vec<_> = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|x| x == "md")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_owned())
        .collect();

    paths.sort_by(|a, b| b.cmp(a));

    paths
        .iter()
        .map(|path| {
            let raw = fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

            let parsed = matter.parse_with_struct::<StoryFrontmatter>(&raw)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: missing or invalid frontmatter (need `title` and `slug`)",
                        path.display()
                    )
                });

            Story {
                title: parsed.data.title,
                slug: parsed.data.slug,
                html_content: md_to_html(&parsed.content),
            }
        })
        .collect()
}

// ── Embedded CSS ──────────────────────────────────────────────────────────────

fn css() -> &'static str {
    r#"
*, *::before, *::after { box-sizing: border-box; }

:root {
  --bg: #0a0a0f;
  --surface: #12121a;
  --border: #1e1e2e;
  --text: #c8c8d8;
  --text-muted: #6868a0;
  --accent: #7c6af7;
  --accent-dim: #3d3478;
  --link: #9d8fff;
}

html { scroll-behavior: smooth; }

body {
  background: var(--bg);
  color: var(--text);
  font-family: 'Georgia', 'Times New Roman', serif;
  font-size: 1.05rem;
  line-height: 1.75;
  margin: 0;
  padding: 0;
}

.site-header {
  text-align: center;
  padding: 4rem 2rem 2rem;
  border-bottom: 1px solid var(--border);
}

.site-header h1 {
  font-size: 2.2rem;
  letter-spacing: 0.04em;
  color: #e8e8f8;
  margin: 0 0 0.5rem;
}

.site-header p {
  color: var(--text-muted);
  font-style: italic;
  margin: 0;
}

main {
  max-width: 680px;
  margin: 0 auto;
  padding: 3rem 2rem;
}

.story {
  margin-bottom: 4rem;
  padding-bottom: 4rem;
  border-bottom: 1px solid var(--border);
  scroll-margin-top: 2rem;
}

.story:last-child {
  border-bottom: none;
}

.story.targeted {
  animation: highlight-fade 2s ease forwards;
}

@keyframes highlight-fade {
  0%   { background: rgba(124, 106, 247, 0.08); border-radius: 6px; }
  100% { background: transparent; }
}

.story-title {
  display: flex;
  align-items: baseline;
  gap: 0.5em;
  margin: 0 0 1.5rem;
}

.story h2 {
  font-size: 1.4rem;
  color: #e8e8f8;
  margin: 0;
  letter-spacing: 0.02em;
}

.copy-link {
  flex-shrink: 0;
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.15em 0.25em;
  color: var(--text-muted);
  transition: color 0.15s;
  line-height: 1;
}

.copy-link:hover {
  color: var(--accent);
}

.copy-link.copied {
  color: #5ecb8a;
}

.copy-confirm {
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  color: #5ecb8a;
  opacity: 0;
  transition: opacity 0.15s;
  white-space: nowrap;
  align-self: center;
}

.copy-confirm.visible {
  opacity: 1;
}

@keyframes confirm-fade {
  0%   { opacity: 1; }
  70%  { opacity: 1; }
  100% { opacity: 0; }
}

.copy-confirm.fading {
  animation: confirm-fade 1.8s ease forwards;
}

.story p {
  margin: 0 0 1rem;
}

.story p:last-of-type {
  margin-bottom: 1.25rem;
}

.share-link {
  display: inline-flex;
  align-items: center;
  gap: 0.4em;
  font-family: 'Courier New', monospace;
  font-size: 0.78rem;
  color: var(--text-muted);
  text-decoration: none;
  border: 1px solid var(--accent-dim);
  border-radius: 3px;
  padding: 0.2em 0.6em;
  transition: color 0.15s, border-color 0.15s;
}

.share-link:hover {
  color: var(--accent);
  border-color: var(--accent);
}

.share-link::before {
  content: '#';
  opacity: 0.5;
}

.site-footer {
  text-align: center;
  padding: 2rem;
  border-top: 1px solid var(--border);
  color: var(--text-muted);
  font-size: 0.9rem;
  font-style: italic;
}

.site-footer hr { display: none; }

a { color: var(--link); }

em { color: #b0b0d0; }

strong { color: #e0e0f0; }

@media (max-width: 600px) {
  .site-header h1 { font-size: 1.6rem; }
  main { padding: 2rem 1.25rem; }
}
"#
}

// ── Embedded JS ───────────────────────────────────────────────────────────────

fn js() -> &'static str {
    r#"
// Highlight the targeted story when navigating to a fragment URL
function highlightTarget() {
  document.querySelectorAll('.story').forEach(el => el.classList.remove('targeted'));
  const id = location.hash.slice(1);
  if (id) {
    const el = document.getElementById(id);
    if (el) el.classList.add('targeted');
  }
}
window.addEventListener('hashchange', highlightTarget);
document.addEventListener('DOMContentLoaded', highlightTarget);

// Copy share link to clipboard
document.addEventListener('click', e => {
  const btn = e.target.closest('.copy-link');
  if (!btn) return;
  const slug = btn.dataset.slug;
  const url = location.origin + location.pathname + '#' + slug;
  navigator.clipboard.writeText(url).then(() => {
    const confirm = btn.parentElement.querySelector('.copy-confirm');
    btn.classList.add('copied');
    confirm.classList.remove('fading');
    confirm.classList.add('visible', 'fading');
    confirm.addEventListener('animationend', () => {
      confirm.classList.remove('visible', 'fading');
      btn.classList.remove('copied');
    }, { once: true });
  });
});
"#
}

// ── HTML assembly ─────────────────────────────────────────────────────────────

fn render_story(story: &Story) -> String {
    let slug = &story.slug;
    let title = html_escape(&story.title);
    let content = story.html_content.trim();
    // SVG: two chain-link rings
    let icon = r#"<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>"#;
    format!(
        "<article class=\"story\" id=\"{slug}\">\n  <div class=\"story-title\">\n    <h2>{title}</h2>\n    <button class=\"copy-link\" data-slug=\"{slug}\" title=\"Copy link\" aria-label=\"Copy link to {title}\">{icon}</button>\n    <span class=\"copy-confirm\">link copied</span>\n  </div>\n  {content}\n  <a class=\"share-link\" href=\"#{slug}\">{slug}</a>\n</article>"
    )
}

fn build_page(header_md: &str, footer_md: &str, stories: &[Story]) -> String {
    let header_html = md_to_html(header_md);
    let footer_html = md_to_html(footer_md);

    let stories_html: String = stories
        .iter()
        .map(render_story)
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Token Effort</title>
  <meta property="og:title" content="Token Effort">
  <meta property="og:description" content="Short fiction from a world trading on tokens and bandwidth.">
  <meta property="og:type" content="website">
  <meta property="og:url" content="https://token-effort.ai/">
  <meta property="og:image" content="https://token-effort.ai/icon-x-small.png">
  <meta property="og:image:width" content="147">
  <meta property="og:image:height" content="148">
  <meta property="og:image:alt" content="Abstract fractal network">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="Token Effort">
  <meta name="twitter:description" content="Short fiction from a world trading on tokens and bandwidth.">
  <meta name="twitter:image" content="https://token-effort.ai/icon-x-small.png">
  <style>{css}</style>
</head>
<body>
<header class="site-header">
  {header}
</header>
<main>
  {stories}
</main>
<footer class="site-footer">
  {footer}
</footer>
<script>{js}</script>
</body>
</html>"#,
        css = css(),
        js = js(),
        header = header_html.trim(),
        stories = stories_html,
        footer = footer_html.trim(),
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let content_dir = "content";
    let stories_dir = "content/stories";
    let output_path = "output/index.html";

    let header_md = fs::read_to_string(Path::new(content_dir).join("header.md"))
        .expect("missing content/header.md");
    let footer_md = fs::read_to_string(Path::new(content_dir).join("footer.md"))
        .expect("missing content/footer.md");

    let stories = load_stories(stories_dir);

    println!("Building {} stories...", stories.len());
    for s in &stories {
        println!("  #{} — {}", s.slug, s.title);
    }

    let html = build_page(&header_md, &footer_md, &stories);

    fs::write(output_path, &html)
        .unwrap_or_else(|e| panic!("failed to write {output_path}: {e}"));

    fs::copy("content/icon-x-small.png", "output/icon-x-small.png")
        .unwrap_or_else(|e| panic!("failed to copy icon-x-small.png: {e}"));

    println!("Done → {output_path}  ({} bytes)", html.len());
}
