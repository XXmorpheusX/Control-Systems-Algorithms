import json
import matplotlib.pyplot as plt
import paho.mqtt.client as mqtt

x = []
y = []
z = []


def on_message(client, userdata, message):
    #print(str(message.payload))

    if message.topic == "CTRL/out2d":
        data = json.loads(message.payload)
        x.append(data["x"]["x"])
        y.append(data["v"]["x"])
        return

    if message.topic == "CTRL/out":
        data = json.loads(message.payload)
        x.append(data["x"]["x"])
        y.append(data["x"]["y"])
        z.append(data["x"]["z"])
        return

    if message.topic == "CTRL/end2d":
        print("plotting 2d...")
        plt.plot(x, y)
        plt.show()

        x.clear()
        y.clear()
        z.clear()
        return

    if message.topic == "CTRL/end":
        print("plotting...")
        ax = plt.figure(figsize=(10,10)).add_subplot(projection='3d')
        ax.plot3D(x, y, z, 'blue')
        ax.set_xlabel("X Axis")
        ax.set_ylabel("Y Axis")
        ax.set_zlabel("Z Axis")
        plt.show()

        x.clear()
        y.clear()
        z.clear()
        return


if __name__ == "__main__":
    print("Setting up mqtt connection")
    mqttc = mqtt.Client()

    mqttc.connect("localhost")
    print("Successfully connected to mqtt broker")

    print("Subscribing to output topics")
    mqttc.subscribe("CTRL/out2d", 2)
    mqttc.subscribe("CTRL/end2d", 2)
    mqttc.subscribe("CTRL/out", 2)
    mqttc.subscribe("CTRL/end", 2)

    print("Setting Callbacks...")
    mqttc.on_message = on_message

    print("Looping...")
    mqttc.loop_forever()
