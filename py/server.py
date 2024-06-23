from flask import Flask, request, jsonify
from flask_cors import CORS
import json

app = Flask(__name__)
CORS(app)  # 允许所有来源

@app.route('/get-content', methods=['GET'])
def get_content():
    return jsonify({"message": "Hello from Python!"})

@app.route('/save-content', methods=['POST'])
def save_content():
    data = request.json
    with open('data.json', 'w') as json_file:
        json.dump(data, json_file)
    return jsonify({"status": "success"})

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=5000)
