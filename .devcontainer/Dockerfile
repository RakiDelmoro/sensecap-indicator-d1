FROM espressif/idf:v5.0.2

# Install additional tools
RUN apt-get update && apt-get install -y \
    git \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspaces/sensecap-indicator-d1

# Note: ESP-IDF is already installed at /home/esp/esp-idf/
# Run '. /home/esp/esp-idf/export.sh' to activate it
