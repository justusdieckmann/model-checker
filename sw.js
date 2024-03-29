const cacheName = 'model_checker_ui-pwa';
const filesToCache = [
/*  './',
  './index.html',
  './model_checker_ui.js',
  './model_checker_ui_bg.wasm',*/
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
