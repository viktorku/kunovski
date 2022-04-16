let refreshTimeout = 0;

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
  document.getElementById('links').classList.toggle('active', true);
};

const MAX_U32 = 4294967295;
const next_u32 = (min = 0, max = MAX_U32) => {
  max = max > MAX_U32 ? MAX_U32 : max;
  return Math.floor(Math.random() * (max - min + 1)) + min;
};

export { next_u32, update, done, refreshTimeout };
