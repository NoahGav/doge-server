import { dev } from "$app/environment";

const base_url = dev ? "http://localhost:3000" : "";

export async function get_transactions() {
    const response = await fetch(`${base_url}/api/transactions`);
    const data = await response.json();
    return data;
}