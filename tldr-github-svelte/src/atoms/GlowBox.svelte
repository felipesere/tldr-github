<script>
  export let content;
</script>

<div class="glow-box grow">
  <a href={content.link}>{content.title}</a>
  <p class="is-size-7">by {content.by}</p>
</div>

<style>
.glow-box {
  padding: 10px;
  background-color: #F8F8F8;
  border: 1px solid #DDDDDB;
  border-radius: 3px;
}

.glow-box:hover {
  background-color: #f6f8fa;
}
</style>
