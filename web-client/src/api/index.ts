function getApiUrl(): string {
	return import.meta.env.VITE_API_URL;
}

async function get() {
	const res = await fetch(`${getApiUrl()}/`, {
		method: "GET",
		mode: 'cors',
	});
	return await res.json();
}

async function post<T>(url: string, data: T) {
	const res = await fetch(`${getApiUrl()}/${url}`, {
		method: "POST",
		mode: 'cors',
		body: JSON.stringify(data),
		credentials: 'include',
		headers: {
			"Content-Type": "application/json",
		},
	});
	return await res.json();
}


export function googleSignIn(credential: string) {
	post('auth/login', { credential });
}