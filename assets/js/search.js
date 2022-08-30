function init() {
  focusInput();
}

function search(text) {
  if (text.length > 0) {
    text = encodeURIComponent(text);
    window.location.href='https://www.google.co.jp/search?q=' + text;
  }
}

function getText() {
  return document.getElementById('q').value;
}

function searchFromInput(e) {
  if (e.which === 13) {
    search(getText());
  }
}

function focusInput() {
  document.getElementById('q').focus();
}
