# Start with python dev image
FROM python:3.11

RUN mkdir -p /opt/project

# Copy in the current directory
COPY . /opt/project

# Set the working directory
WORKDIR /opt/project

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install dependencies
RUN pip install -r requirements.txt
RUN pip install .

