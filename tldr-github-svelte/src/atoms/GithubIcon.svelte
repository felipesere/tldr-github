<script>
    // because jest+babel and svelte can't agree on imports
    import * as octicons from '@primer/octicons'

    let allIcons = octicons.default || octicons;

    export let icon = ""
</script>


{#if !icon }
    <!-- nothing -->
{:else}
    {@html allIcons[icon].toSVG()}
{/if}


<style global>
    .octicon {
        display: inline-block;
    }
</style>
