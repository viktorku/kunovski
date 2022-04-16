const wa = import('./kunovski');
const { refreshTimeout } = require('./snippets')

const re = document.getElementById('refresh');

wa.then(wa => {

  // Hack! Fixme. So that update in snippets can schedule a timeout
  window.start = (options) => {
    const partial = options && options.partial;
    const scheduled = options && options.schedules;
    if (!scheduled) {
      clearTimeout(refreshTimeout);
    }
    wa.start(partial);
  };

  // Initialize the corpus (takes up to a second).
  wa.init();

  const defer = timeout => {
    setTimeout(() => {
      start();
    }, timeout);
  };

  const visChange = () => {
    if (!document.hidden) {
      defer(500);
    }
  };

  if (document.hidden) {
    document.addEventListener("visibilitychange", visChange);
  } else {
    start();
  }

  re.addEventListener('click', e => {
    e.preventDefault();
    document.getElementById('links').classList.toggle('active', false);
    start();
  }, false);

});
