// A little client-side scripting for smooth UI

// Trigger a refresh of all sources
const refreshSources = () => {
  // check if it's time yet - also validated server-side
  const last_refresh = document.querySelector("#last-refresh").textContent;
  const now = Date.now();
  if (last_refresh === "never" || now - new Date(last_refresh) > 1000 * 60 * 60 * 12) {
    const Http = new XMLHttpRequest();
    const url = "/refresh";
    Http.open("PUT", url);
    Http.send();
    Http.onreadystatechange = _ => window.location.reload(true);
  }
};

// Call on load
window.addEventListener("DOMContentLoaded", _ => refreshSources());
