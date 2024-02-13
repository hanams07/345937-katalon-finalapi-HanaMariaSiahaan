<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateBooking</name>
   <tag></tag>
   <elementGuidId>1651edb1-7a1f-48bf-a22d-e992cee1e88c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${firstname}\&quot;,\n    \&quot;lastname\&quot; : \&quot;${lastname}\&quot;,\n    \&quot;totalprice\&quot; : ${totalprice},\n    \&quot;depositpaid\&quot; : ${depositpaid},\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;${checkin}\&quot;,\n        \&quot;checkout\&quot; : \&quot;${checkout}\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;${additionalneeds}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>452463c9-cf4b-4ca5-812b-b778f370850c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>4c15ea9c-4f7c-473f-bebc-6f1b2bda2878</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'siahaan'</defaultValue>
      <description></description>
      <id>db94b045-70f4-46af-bf02-c27bd0aa2b64</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'juntak'</defaultValue>
      <description></description>
      <id>af1d0c03-e60b-403a-bb2a-36701beb68e8</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>180</defaultValue>
      <description></description>
      <id>d69de14c-6b71-4901-9bab-4edda7669733</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>f90b0913-f956-4242-b198-f15b83564a92</id>
      <masked>false</masked>
      <name>depositpaid</name>
   </variables>
   <variables>
      <defaultValue>'2024-02-10'</defaultValue>
      <description></description>
      <id>501f900e-2e2e-4dc0-b233-ddd4084973aa</id>
      <masked>false</masked>
      <name>checkin</name>
   </variables>
   <variables>
      <defaultValue>'2024-02-19'</defaultValue>
      <description></description>
      <id>131ae8fe-a417-43fb-aeef-44feb242e21f</id>
      <masked>false</masked>
      <name>checkout</name>
   </variables>
   <variables>
      <defaultValue>'Prabowo'</defaultValue>
      <description></description>
      <id>54a77d5a-d69b-45f9-a894-c1cfdfbdf40c</id>
      <masked>false</masked>
      <name>additionalneeds</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
