<script>
    import {search} from './search'
    import GithubIcon from "../atoms/GithubIcon.svelte";

    export let items;
    export let searchResults = items;
    let fields = ["title", "by", "labels"];
    let term = "";

    $: searchTerm = new RegExp(term, "i");
    $: searchResults = search(items, searchTerm, fields)
</script>

<div class="flex flex-row">
    <span class="w-auto flex justify-end items-center text-gray-500 p-2 fill-current">
        <GithubIcon icon="search"/>
    </span>
    <input bind:value={term}
           class="w-full rounded p-2 bg-gray-200 hover:border-gray-300 focus:outline-none focus:bg-white focus:shadow-outline focus:border-blue-300"
           type="text" placeholder="Search..."/>
</div>
