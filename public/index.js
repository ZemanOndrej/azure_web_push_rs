function urlBase64ToUint8Array(base64String) {
	const padding = '='.repeat((4 - (base64String.length % 4)) % 4);
	const base64 = (base64String + padding).replace(/\-/g, '+').replace(/_/g, '/');

	const rawData = window.atob(base64);
	const outputArray = new Uint8Array(rawData.length);

	for (let i = 0; i < rawData.length; ++i) {
		outputArray[i] = rawData.charCodeAt(i);
	}
	return outputArray;
}

async function handleNotificationPermission(params) {
	if (!('Notification' in window)) {
		// Check if the browser supports notifications
		alert('This browser does not support desktop notification');
	} else if (Notification.permission === 'granted') {
		console.log('Notification.permission', Notification.permission);
	} else if (Notification.permission !== 'denied') {
		let permission = await Notification.requestPermission();
		console.log('Notification.permission', permission);
	}
}

async function getPNSSubscription() {
	const publicVapidKey = await fetch('/public/public.key').then(res => res.text());

	const registration = await navigator.serviceWorker.register('./public/service-worker.js');
	console.log('Service Worker registered with scope:', registration);

	let subscription = await registration.pushManager.getSubscription();
	console.log('subscription', subscription);

	if (subscription === null) {
		// Subscribe the user
		subscription = await registration.pushManager.subscribe({
			userVisibleOnly: true,
			applicationServerKey: urlBase64ToUint8Array(publicVapidKey)
		});
	}
	return subscription;
}

async function handleSendNotificationClick() {
	console.log('send notification');

	try {
		let result = await fetch('/api/notification', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			}
		});
		let body = await result.json();

		console.log('Notification sent:', body);
		resultDiv.innerHTML = JSON.stringify(body, null, 2);
	} catch (error) {
		console.log('Notification sent:', error);
		resultDiv.innerHTML = JSON.stringify(error, null, 2);
	}
}

async function handleGetRegistrations() {
	console.log('get registrations');
	try {
		let result = await fetch('/api/registrations', {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		let body = await result.json();
		console.log('registrations :', body);
		resultDiv.innerHTML = JSON.stringify(body, null, 2);
	} catch (error) {
		resultDiv.innerHTML = JSON.stringify(error, null, 2);
	}
}

async function unregisterServiceWorkers() {
	let registrations = await navigator.serviceWorker.getRegistrations();
	for (const registration of registrations) {
		registration.unregister();
	}
}

export async function run() {
	try {
		await handleNotificationPermission();

		if (!('serviceWorker' in navigator)) {
			alert('Service Workers not supported');
			return;
		}

		let subscription = await getPNSSubscription();

		console.log('User is subscribed:', subscription);

		// Send subscription to your server
		let res = await fetch('/api/registration', {
			method: 'POST',
			body: JSON.stringify(subscription),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		let body = await res.text();
		document.getElementById('text_response_field').innerText = JSON.stringify(body, null, 2);
	} catch (error) {
		console.error('Service Worker Error', error);
	}
}

document.getElementById('send_notification').addEventListener('click', handleSendNotificationClick);
document.getElementById('btn_get_registrations').addEventListener('click', handleGetRegistrations);
document.getElementById('btn_unregister_sw').addEventListener('click', unregisterServiceWorkers);
document.getElementById('btn_register_sw').addEventListener('click', run);

let resultDiv = document.getElementById('text_response_field');
