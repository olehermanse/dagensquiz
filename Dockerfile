FROM python:3-alpine
WORKDIR /dagensquiz
ADD ./ /dagensquiz
RUN pip3 install -r requirements.txt
ENV PORT 80
EXPOSE 80
CMD ["gunicorn", "-b", "0.0.0.0:80", "run:app"]
