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
		resultDiv.innerHTML = JSON.stringify(body);
	} catch (error) {
		console.log('Notification sent:', error);
		resultDiv.innerHTML = JSON.stringify(error);
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
		resultDiv.innerHTML = JSON.stringify(body);
	} catch (error) {
		resultDiv.innerHTML = JSON.stringify(error);
	}
}

document.getElementById('send_notification').addEventListener('click', handleSendNotificationClick);
document.getElementById('btn_get_registrations').addEventListener('click', handleGetRegistrations);

let resultDiv = document.getElementById('text_response_field');
