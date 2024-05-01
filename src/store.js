// in a file named store.js or any appropriate file
import { writable, derived } from 'svelte/store';

// Define a writable store for selectedName
export const graphNameStore = writable('blank');

// Create a derived store that reacts to changes in selectedName
export const graphName = derived(graphNameStore, ($graphNameStore) => $graphNameStore);
