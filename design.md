

                                     M_POS

    --------------------------------------------------------------------------
                         DART UI  (Web, Android, iOS)
    -------------------------------------------------------------------------- 
                          Blockchain Transaction Log 
    --------------------------------------------------------------------------  
              Security | Data | Communications Layer (Native / Rust)
    --------------------------------------------------------------------------


    Key Features: Multi-Platform, local/IOT device with:
        0) The first POS system with secure, memory-safe, cryto-payment integration
        1) Optional cloud synchronization,
        2) Tamper-proof transaction history,
        3) Extreme performance,
        4) Optional low-cost integrated device,
        5) Open-source code,
        6) Peer-to-peer auto-sync for inventory 
        7) Integrated food delivery/ordering api
        8) Integrated customer self-ordering menu 
        9) Extensible business rules for reporting and financials
        10) Standard SDK for IoT/Cloud/Storage provided as a module


## DESIGN:

```
                                      3rd Party API
        
PROTOCOL:                      GRPC         |       ?
              cli     ->                                                -> Dgraph
APPLICATION:          client  <-------> API - N Servers <-------> ?
              flutter ->                                                -> Sled 
                        \                   |                   /

TELEMETRY:                  Telemetry Router / Sink / Vector

```

### Elementary of the module

  ![Elementary Module](https://github.com/xenirio/essembly/blob/master/assets/essembly-elementary-design.svg)

  For every module there are 3 entry point.
  1) `command` - the `CLI` to interact the module with command set.
  2) `communication` - the `gRPC` protocal to describe funcational/interface of the module by `.proto` file.
  3) `monitoring` - the `Log & Telemery` details in realtime & post-processing logs.
  
  In each module there is a router to be the main interaction of internal module via `gRPC` protocol.  
  In each module can contains one or many module inside.


## Use Cases:

1) I have a restaurant and I want to track my sales (traditional POS)
2) I have a restaurant and I want to offer delivery via a 3rd party service (Grab, Panda, etc.) 
3) I have 3 restaurants and I want to know inventory levels at all of them. 
4) I have a restaurant and I want to use a 3rd party ordering api (e.g. baked goods I do not make in-house). 
5) I have a at-home kitchen and want to offer my products to restaurants.
6) I have a at-home kitchen and want to offer my products directly to consumers.
7) I have a delivery service and want to receive orders and dispatch my drivers and track deliveries. 
8) When the customer do the reservation and we reserve seat with customer's order.
9) When the resturant don't have available seat or some food need to be cooked. User will get notification 15-20 min before it's ready.
10) Customer need to order on their phone by scan the QR code.
11) Customer need to know how long the order will take and when the menu ready to serve.
12) He can make a surprise cake for dating by secret request to the kitchen.

## Design CI/CD Rules:

1) Compile all business rules to wasm to use for in-place updates - distributed http updates?
2)   


## Blockchain

1) If you have a shop, and someone wants to buy with Lite Coin or Ether, what if you could accept it, and charge a tiny free for the transaction?integration with crypto.com's platform (or something like it) could let us optionally offer that to users with near real-time transactions.In countries where it is still not legal, disable it. You are effectively making each merchant a crypto-broker.

2) Business logic within an enterprise - multiple facility implementation - can have a canonical ledger to reference and share (e.g. inventory)



## Other Notes:

## Accounting Systems:
1) Express accounting software 
2) QuickBooks accounting 
3) ACCPAC Accounting System 
4) SAP accounting software 
