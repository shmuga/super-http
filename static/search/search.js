function updateSearch(value) {
  document.getElementById("search").value = value;
}
function updateLink(value) {
  const link = `/search?q=${value}`;
  document.getElementById("search-link").href = link;
  return link;
}

function renderResult({ tag = [], title = [], body = [], filename = [] }) {
  var [s, t] = [...new Set(title)];
  if (t == undefined) t = s;

  return `
    <div class="result">
      <a href="${filename[0]}">
        <h3>${t}</h3>
        <span class="result-subtitle">${s}</span>
        <span class="result-path">${filename[0]}</span>
      </a>
      <div class="result-body">
        ${body}
      </div>
      <div class="result-tags">
        ${tag.map(t => `<div class="tag">${t}</div>`).join('')}
      </div>
    </div>
  `;
}
function updateResults(data) {
  document.getElementById("results-box").innerHTML = data.map(renderResult).join('');
}
document.getElementById("search").addEventListener("change", (event) => {
  updateLink(event.target.value);
});

document.getElementById("search").addEventListener("keydown", (event) => {
  if (event.key == 'Enter') {
    const link = updateLink(event.target.value);
    window.location = link;
  }
});

window.onload = function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const q = urlParams.get("q");

  if (q) {
    updateSearch(q);
    updateLink(q);
    fetch(`/api/search/${q}`)
      .then((d) => d.json())
      .then(updateResults);
  }
};
