
import { writable } from 'svelte/store';

export const user = writable<User>(null);

export interface User {
	name: string
}