<script>
    import {search} from './search'

    export let items;
    export let searchResults = items;
    let fields = ["title", "by", "labels"];
    let term = "";

    const suppressed = () => {};

    $: searchTerm = new RegExp(term, "i");
    $: searchResults = search(items, searchTerm, fields)
</script>

<form class="appearance-none shadow-inline border rounded py-2 px-3 text-gray-700 leading-tight" on:submit|preventDefault={suppressed}>
    <input bind:value={term} class="focus:border-blue-600 focus:text-black focus:outline-none" type="text" placeholder="Search..." />
</form>
