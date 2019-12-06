<script>
  import GlowBox from './GlowBox.svelte';
  export let issues;
</script>


<div>
  {issues.length} new Issues:
  {#each issues as issue}
    <GlowBox content={issue} />
  {/each}
</div>
