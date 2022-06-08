# Light-MP-Serde

A different approach to serializing and deserializing messages in message pack.

rmp-serde is a library for serializing and deserializing messages in message pack. However, it packs array headers
around all data.

This library will not pack array headers around data. and then serialize the data straight into byte array