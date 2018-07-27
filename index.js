const wa = import('./kunovski');

const re = document.getElementById('refresh');
const links = document.getElementById('links');

let refreshTimeout = 0;
let start;

wa.then(wa => {

  start = (options) => {
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
    links.classList.toggle('active', false);
    start();
  }, false);

});

const update = (status, _pending) => {
  // Schedule partial-auto-refresh when pending char-rolling is done
  if (status && typeof start === 'function') {
    refreshTimeout = setTimeout(() => {
      start({
        partial: true,
        scheduled: true,
      });
    }, 30000);
  }
  return !status;
};

const done = () => {
  links.classList.toggle('active', true);
};

const MAX_U32 = 4294967295;
const next_u32 = (min = 0, max = MAX_U32) => {
  max = max > MAX_U32 ? MAX_U32 : max;
  return Math.floor(Math.random() * (max - min + 1)) + min;
};

export { next_u32, update, done };
