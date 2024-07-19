<template>
  <div class="container">

    <div class="msg_area">
      <div class="msg_card" v-for="msg in data.msgList">{{ msg.content }}</div>
    </div>

    <div class="input_area">

      <a-button :disabled="data.sending" class="circle_btn" type="primary" shape="circle" size="large">
        <template #icon>
          <UploadOutlined />
        </template>
      </a-button>


      <a-textarea :disabled="data.sending" class="input_box" v-model:value="value2"
        placeholder="输入你的问题,使用Shift+Enter快速发送"
        :auto-size="{ minRows: 2, maxRows: 5 }" />


      <a-button @click="toggleDis(true)" :disabled="data.sending" class="circle_btn" type="primary" shape="circle"
        size="large">
        <template #icon>
          <SendOutlined />
        </template>
      </a-button>


    </div>
  </div>
</template>
<script setup>
import { ref } from 'vue';
import { UploadOutlined, SendOutlined } from '@ant-design/icons-vue';

let data = ref({
  sending: false,
  msgList: [
    {
      role: "user",
      avatar:"",
      content: "xxxxxxxxxxxxxxxxxx ###6666"
    },
  ]
});

const toggleDis = (state) => {
  data.value.sending = state;
}
</script>
<style scoped lang="scss">
.container {
  width: 100%;
  height: 100%;
  background: #f7f8fc;

  .msg_area {
    width: 100%;
    height: 100%;

    .msg_card {
      border: 1px solid #ccc;
      width: 100%;
      min-height: 100px;
    }
  }

  .input_area {
    width: 100%;
    height: 100px;
    position: fixed;
    left: 0;
    bottom: 0;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    z-index: 100;
    opacity: 1;

    .circle_btn {
      width: 50px;
      height: 50px;
    }

    .input_box {
      width: 80%;
    }

  }
}
</style>