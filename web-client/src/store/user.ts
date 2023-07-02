
import { writable } from 'svelte/store';

export const user = writable<User>(null);

export interface User {
	first_name: string;
	last_name: string;
	email: string;
}