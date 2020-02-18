// A little client-side scripting for smooth UI

// Trigger a refresh of all sources
const refreshSources = () => {
  const Http = new XMLHttpRequest();
  const url = "/refresh";
  Http.open("PUT", url);
  Http.send();
  Http.onreadystatechange = _ => window.location.reload(true);
};