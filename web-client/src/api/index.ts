import { push, pop, replace } from 'svelte-spa-router'
import { user, type User } from '../store/user';

function getApiUrl(): string {
	return import.meta.env.VITE_API_URL;
}

function getAuthToken(): string {
	return localStorage.getItem("authToken");
}

async function get<Response>(url: string) {
	const res = await fetch(`${getApiUrl()}/${url}`, {
		method: "GET",
		mode: 'cors',
		headers: {
			"Authorization": `Bearer ${getAuthToken()}`
		}
	}).then((res) => {
		checkErrors(res);
		return res;
	});

	return await res.json() as Response;
}

async function post<T, Response>(url: string, data: T) {
	const res = await fetch(`${getApiUrl()}/${url}`, {
		method: "POST",
		mode: 'cors',
		body: JSON.stringify(data),
		credentials: 'include',
		headers: {
			"Content-Type": "application/json",
		},
	}).then(res => {
		checkErrors(res);
		return res;
	});
	return await res.json() as Response;
}

function checkErrors(res: Response) {
	if (res.status >= 400 && res.status <= 499) {
			push("/login");
			throw res;
	}
}


export function googleSignIn(credential: string) {
	post<{ credential: string }, GoogleSignInRes>('auth/login', { credential }).then((res) => {
		localStorage.setItem('authToken', res.token)
		push("/")
	});
}

export async function me(): Promise<User> {
	return get<User>('me').then(val => {
		user.set(val);
		return val;
	});
}

interface GoogleSignInRes {
	token: string
}

