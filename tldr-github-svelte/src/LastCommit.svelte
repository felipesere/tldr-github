<script>
  export let lastCommit;
</script>

<div>
  Last commit on <code>{lastCommit.branch}</code> {lastCommit.on} by {lastCommit.by}:
  <div class="commit">{lastCommit.comment}</div>
</div>

<style>
.commit {
  margin-top: 4px;
  padding: 10px;
  color: #586069;
  background-color: #f1f8ff;
  border: 1px solid #c8e1ff;
}
</style>  
