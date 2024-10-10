

self.addEventListener('push', function (event) {
	self.console.log('Push received');
	const data = event.data.json();
	console.log('Push data:', data);
	const options = {
		body: data.body,
		icon: 'icon.png', // Path to an icon image
		badge: 'badge.png' // Path to a badge image
	};
	event.waitUntil(
		self.registration.showNotification(data.title, options)
	);
});