<script setup>
import nowifi from "../svg/nowifi.vue";
import cross from "../svg/cross.vue";

import { defineProps } from "vue";

const props = defineProps({
    name:String,
    data:String
})

</script>
<template>
     <div class="vpn-config">
            <div class="config-icon">
              <nowifi/>
            </div>
          
            <div class="config-details">
              <span class="config-name">{{ name }}</span>
              <span class="config-protocol">{{data}}</span>
            </div>

            <div class="config-actions">
              <button class="connect-btn">Connect</button>
              <button class="delete-btn">
                <cross></cross>
              </button>
            </div>
          </div>
</template>

<style scoped lang="scss">
.vpn-config {
  background: #353550;
  border: 1px solid rgba(114, 129, 225, 0.2);
  border-radius: 0.75rem;
  padding: 1rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  
  .config-icon {
    flex-shrink: 0;
  }
  
  .config-details {
    flex: 1;
    min-width: 0;
    
    .config-name {
      display: block;
      font-weight: 500;
      margin-bottom: 0.25rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    
    .config-protocol {
      color: rgba(255, 255, 255, 0.7);
      font-size: 0.875rem;

      display: -webkit-box;
      -webkit-line-clamp: 2;          
      -webkit-box-orient: vertical;
      overflow:hidden;
    }
  }
  
  .config-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
}

</style>