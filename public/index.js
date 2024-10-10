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

document.getElementById('send_notification').addEventListener('click', handleSendNotificationClick);
document.getElementById('btn_get_registrations').addEventListener('click', handleGetRegistrations);

let resultDiv = document.getElementById('text_response_field');
