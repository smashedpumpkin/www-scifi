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

// ── Preview image ─────────────────────────────────────────────────────────────

fn preview_svg() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="630" viewBox="0 0 1200 630">
  <defs>
    <filter id="a" x="-30%" y="-30%" width="160%" height="160%">
      <feTurbulence type="turbulence" baseFrequency="0.018" numOctaves="4" seed="7" result="n"/>
      <feDisplacementMap in="SourceGraphic" in2="n" scale="28" xChannelSelector="R" yChannelSelector="G"/>
    </filter>
    <filter id="b" x="-60%" y="-60%" width="220%" height="220%">
      <feGaussianBlur stdDeviation="5"/>
    </filter>
    <filter id="c" x="-60%" y="-60%" width="220%" height="220%">
      <feGaussianBlur stdDeviation="2.5" result="k"/>
      <feMerge><feMergeNode in="k"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    <radialGradient id="pg" cx="38%" cy="52%" r="48%">
      <stop offset="0%" stop-color="#2a1860" stop-opacity=".75"/>
      <stop offset="100%" stop-color="#0a0a0f" stop-opacity="0"/>
    </radialGradient>
    <radialGradient id="gg" cx="78%" cy="72%" r="38%">
      <stop offset="0%" stop-color="#082818" stop-opacity=".8"/>
      <stop offset="100%" stop-color="#0a0a0f" stop-opacity="0"/>
    </radialGradient>
  </defs>

  <rect width="1200" height="630" fill="#0a0a0f"/>
  <rect width="1200" height="630" fill="url(#pg)"/>
  <rect width="1200" height="630" fill="url(#gg)"/>

  <!-- Grid -->
  <g stroke="#18182e" stroke-width="1" opacity=".8">
    <line x1="0"   y1="158" x2="780" y2="158"/>
    <line x1="0"   y1="315" x2="920" y2="315"/>
    <line x1="0"   y1="472" x2="680" y2="472"/>
    <line x1="150" y1="0"   x2="150" y2="630"/>
    <line x1="300" y1="0"   x2="300" y2="580"/>
    <line x1="450" y1="0"   x2="450" y2="510"/>
    <line x1="600" y1="0"   x2="600" y2="400"/>
    <line x1="750" y1="0"   x2="750" y2="315"/>
    <line x1="900" y1="0"   x2="900" y2="200"/>
  </g>

  <!-- Circuit traces -->
  <g stroke="#3d3478" stroke-width="1.5" fill="none" opacity=".55">
    <polyline points="0,315 300,315 300,158 600,158"/>
    <polyline points="150,472 150,315 450,315 450,158"/>
    <polyline points="300,472 450,472 450,315"/>
    <polyline points="600,158 750,158 750,315 900,315"/>
    <polyline points="600,315 600,472"/>
  </g>
  <g fill="#7c6af7" filter="url(#c)">
    <circle cx="300" cy="315" r="5"/> <circle cx="300" cy="158" r="4"/>
    <circle cx="450" cy="158" r="5"/> <circle cx="450" cy="315" r="5"/>
    <circle cx="450" cy="472" r="4"/> <circle cx="600" cy="158" r="5"/>
    <circle cx="600" cy="315" r="4"/> <circle cx="600" cy="472" r="5"/>
    <circle cx="750" cy="158" r="4"/> <circle cx="750" cy="315" r="5"/>
    <circle cx="900" cy="315" r="4"/> <circle cx="150" cy="315" r="4"/>
    <circle cx="150" cy="472" r="4"/>
  </g>

  <!-- Central warped form -->
  <g filter="url(#a)">
    <path d="M520,145 C600,125 700,175 725,285 C750,395 685,465 570,470 C448,476 355,405 340,305 C322,192 415,135 520,145Z"
          fill="none" stroke="#6050a8" stroke-width="2.5" opacity=".7"/>
    <path d="M520,175 C588,158 672,200 692,295 C712,382 658,440 558,444 C448,450 368,390 355,300 C340,202 428,165 520,175Z"
          fill="none" stroke="#4e3e90" stroke-width="1.5" opacity=".55"/>
    <path d="M520,205 C576,191 645,226 660,304 C675,374 628,422 545,425 C452,430 383,378 372,298 C359,214 442,194 520,205Z"
          fill="#110a30" stroke="#3d3170" stroke-width="1" opacity=".5"/>
    <ellipse cx="518" cy="308" rx="105" ry="100" fill="#3d2d80" opacity=".18" filter="url(#b)"/>
  </g>

  <!-- Mangrove tendrils -->
  <g stroke="#1e6840" stroke-width="2" fill="none" stroke-linecap="round" opacity=".8">
    <path d="M970,630 C965,592 980,558 960,520 C942,485 962,450 940,410 C920,374 945,338 922,295"/>
    <path d="M940,410 C912,394 882,404 852,382 C826,364 800,372 768,350 C742,332 712,336 680,315"/>
    <path d="M852,382 C844,356 858,328 840,302 C826,280 840,255 828,228"/>
    <path d="M840,302 C866,290 892,298 920,278 C940,264 960,265 984,248"/>
    <path d="M922,295 C962,278 1000,288 1040,266 C1070,250 1098,254 1132,236"/>
    <path d="M1040,266 C1048,240 1038,214 1052,190 C1062,172 1054,150 1068,128"/>
    <path d="M1052,190 C1080,178 1108,186 1136,166 C1155,152 1168,152 1190,138" stroke-width="1.4"/>
    <path d="M1095,630 C1090,596 1106,566 1088,532 C1073,502 1090,470 1074,435"/>
    <path d="M1074,435 C1102,418 1128,428 1155,408 C1173,394 1185,395 1200,378"/>
    <path d="M680,315  C668,294 678,270 660,246" stroke-width="1.3"/>
    <path d="M828,228  C842,206 834,182 848,158 C856,142 850,124 864,104" stroke-width="1.3"/>
    <path d="M1068,128 C1084,108 1078,86 1094,64" stroke-width="1.3"/>
  </g>

  <!-- Comm-sats -->
  <g fill="#9d8fff" filter="url(#c)">
    <circle cx="58"   cy="42"  r="1.8"/> <circle cx="132"  cy="33"  r="1.5"/>
    <circle cx="208"  cy="46"  r="2"/>   <circle cx="286"  cy="31"  r="1.5"/>
    <circle cx="368"  cy="44"  r="1.8"/> <circle cx="448"  cy="27"  r="1.5"/>
    <circle cx="530"  cy="48"  r="2"/>   <circle cx="614"  cy="34"  r="1.5"/>
    <circle cx="696"  cy="42"  r="1.8"/> <circle cx="780"  cy="28"  r="1.5"/>
    <circle cx="864"  cy="46"  r="2"/>   <circle cx="950"  cy="33"  r="1.5"/>
    <circle cx="1038" cy="48"  r="1.8"/> <circle cx="1124" cy="36"  r="1.5"/>
    <circle cx="92"   cy="86"  r="1.5"/> <circle cx="174"  cy="75"  r="1.8"/>
    <circle cx="258"  cy="90"  r="1.5"/> <circle cx="344"  cy="78"  r="2"/>
    <circle cx="430"  cy="93"  r="1.5"/> <circle cx="516"  cy="72"  r="1.8"/>
    <circle cx="604"  cy="86"  r="1.5"/> <circle cx="690"  cy="75"  r="2"/>
    <circle cx="778"  cy="90"  r="1.5"/> <circle cx="866"  cy="78"  r="1.8"/>
    <circle cx="954"  cy="92"  r="1.5"/> <circle cx="1044" cy="80"  r="2"/>
    <circle cx="1134" cy="88"  r="1.5"/> <circle cx="1188" cy="56"  r="1.8"/>
    <circle cx="155"  cy="128" r="1.4"/> <circle cx="358"  cy="116" r="1.8"/>
    <circle cx="564"  cy="122" r="1.4"/> <circle cx="770"  cy="112" r="2"/>
    <circle cx="978"  cy="126" r="1.5"/>
  </g>

  <!-- Title -->
  <text x="88" y="552" font-family="'Courier New',Courier,monospace" font-size="82" font-weight="bold" letter-spacing="5" fill="#e8e8f8">TOKEN EFFORT</text>
  <text x="90" y="598" font-family="'Courier New',Courier,monospace" font-size="17" letter-spacing="5" fill="#6868a0">SHORT FICTION. TOKENS AND BANDWIDTH.</text>
</svg>"##
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
  <meta property="og:image" content="https://token-effort.ai/preview.svg">
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="Token Effort">
  <meta name="twitter:description" content="Short fiction from a world trading on tokens and bandwidth.">
  <meta name="twitter:image" content="https://token-effort.ai/preview.svg">
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

    fs::write("output/preview.svg", preview_svg())
        .unwrap_or_else(|e| panic!("failed to write output/preview.svg: {e}"));

    println!("Done → {output_path}  ({} bytes)", html.len());
    println!("       output/preview.svg");
}
