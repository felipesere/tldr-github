<script>
    export let content;
</script>

<div class="bg-gray-100 hover:bg-blue-100 flex-grow p-3 border border-gray-300 hover:border-blue-300 rounded-l">
    <a class="text-blue-600" href={content.link}>{content.title}</a>
    <p class="text-sm text-gray-500 font-light">by {content.by}</p>
</div>

