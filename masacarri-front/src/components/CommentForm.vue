<script setup lang="ts">
import type { Comment } from '@/models';
import { ref } from 'vue';
import { useCommentsStore } from '@/CommentsStore';
import { storeToRefs } from 'pinia';


const props = defineProps<{
  comment_replyto?: string
}>();

const store = useCommentsStore();
const { page_id } = storeToRefs(store);

const emit = defineEmits<{
  (e: 'commentSubmit', data: Comment): void
}>();

const comment_form = ref({
  display_name: "",
  site_url: "",
  mail_addr: "",
  content: "",
});
const comment_form_info = ref<string | null>(null);

function submit_comment() {
  if (!page_id.value) {
    comment_form_info.value = "comment submit error";
    return;
  }

  comment_form_info.value = "in progress...";

  const res = store.submitComment({
    reply_to: props.comment_replyto,
    display_name: comment_form.value.display_name,
    site_url: comment_form.value.site_url,
    mail_addr: comment_form.value.mail_addr,
    content: comment_form.value.content,
  }).then((res) => {
    comment_form.value.content = "";
    emit("commentSubmit", res);
  });
}
</script>

<template>
  <form>
    <dl>
      <dt class="required-label">名前</dt>
      <dd><input type="text" name="display_name" required v-model="comment_form.display_name" /></dd>
      <dt>サイトURL</dt>
      <dd><input type="url" name="site_url" v-model="comment_form.site_url" /></dd>
      <dt class="required-label">コメント</dt>
      <dd><textarea rows="3" required v-model="comment_form.content" placeholder="**Markdownが使用できます**"></textarea></dd>
      <dt title="コメント欄上で表示されることはありません">返信通知先</dt>
      <dd><input type="email" name="mail_addr" placeholder="info@example.com" v-model="comment_form.mail_addr" /></dd>
      <button class="comment-submit" type="button" @click="submit_comment">{{ comment_replyto ? "返信" : "送信" }}</button>
    </dl>
  </form>
</template>

<style scoped>
dl {
  display: flex;
  flex-wrap: wrap;
}

dt {
  width: 8em;
  display: block;
  padding: 0.4rem;
}

input,
textarea {
  width: 100%;
  padding: 0.3em;
}

textarea {
  width: 100%;
  height: 100%;
  resize: none;
}

dd {
  width: calc(100% - 8em);
  display: block;
  padding: 0.2em;
}

.required-label:after {
  color: #f22;
  content: '*';
  font-weight: bold;
}

.comment-submit {
  background-color: rgb(13, 139, 97);
  transition: background-color 0.3s ease;
  margin: 0.2rem 0 0.5rem;
  color: #fff;
  width: 100%;
  font-size: 1.2rem;
  border-radius: 0.3rem;
  padding: 0.25rem;
}

.comment-submit:hover {
  background-color: rgb(11, 104, 73);
}
</style>
