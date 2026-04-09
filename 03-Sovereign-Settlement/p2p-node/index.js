import { createLibp2p } from 'libp2p'
import { tcp } from '@libp2p/tcp'
import { mplex } from '@libp2p/mplex'
import { noise } from '@chainsafe/libp2p-noise'
import { mdns } from '@libp2p/mdns'

const createNode = async () => {
  const node = await createLibp2p({
    transports: [tcp()],
    streamMuxers: [mplex()],
    connectionEncryption: [noise()],
    peerDiscovery: [
      mdns({
        interval: 1000 // Scan for other agents every second
      })
    ]
  })

  node.addEventListener('peer:discovery', (evt) => {
    console.log(`🚀 Sovereign Node Found Agent: ${evt.detail.id.toString()}`)
    // Hook: When found, we can query DRI-Index to see if this agent is a fit
  })

  await node.start()
  console.log('🔗 Sovereign Mesh Node Started. ID:', node.peerId.toString())
}

createNode()
