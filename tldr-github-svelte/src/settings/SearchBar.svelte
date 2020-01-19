<script>
    export let items;
    export let fields;
    export let searchResults = items;
    let term = "";

    $: searchTerm = new RegExp(term, "i");

    function matchesFields(item, search, fields) {
        for (const field of fields) {
            const value = item[field];

            if (!value) {
                continue
            }

            if (Array.isArray(value)) {
                if (value.some((v) => search.test(v))) {
                    return true
                }
            }

            if (search.test(value)) {
                return true
            }
        }

        return false
    }

    $: searchResults = items.filter((i) => matchesFields(i, searchTerm, fields))
</script>

<div>
    <form class="field">
        <div class="control">
            <input bind:value={term} class="input" type="text" placeholder="Search..."/>
        </div>
    </form>
</div>