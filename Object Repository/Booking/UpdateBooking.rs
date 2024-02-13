<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateBooking</name>
   <tag></tag>
   <elementGuidId>a51f9744-23e0-47c9-8e8b-3587a6fb862f</elementGuidId>
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
      <webElementGuid>8ceee1e3-3f33-436d-a149-a433f5639983</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a8755a62-ec96-4f34-baf4-bf3e073f63bb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>token=&lt;t7d7e478e6766550></value>
      <webElementGuid>ea570800-e878-4dab-96b0-75c111c3ebb2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YWRtaW46cGFzc3dvcmQxMjM=</value>
      <webElementGuid>a746b997-fbf4-48e4-b44c-8e161fc9f118</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/booking/2276</restUrl>
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
      <id>04593b99-39b5-4467-8dc2-0a6260daff39</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'juntak'</defaultValue>
      <description></description>
      <id>01cc1cb9-32f3-41f4-b0ab-1dcb977bfbcb</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>180</defaultValue>
      <description></description>
      <id>b3839367-bf4c-46a9-9ec5-2dd66f6d4711</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>d0ec397c-8100-4530-9faa-7531179c7e71</id>
      <masked>false</masked>
      <name>depositpaid</name>
   </variables>
   <variables>
      <defaultValue>'2024-02-10'</defaultValue>
      <description></description>
      <id>e987618a-25cb-4dc8-87cb-d1aa3ee2976d</id>
      <masked>false</masked>
      <name>checkin</name>
   </variables>
   <variables>
      <defaultValue>'2024-02-19'</defaultValue>
      <description></description>
      <id>d6d256cb-9bbb-4bf7-9f10-041d0ebe2310</id>
      <masked>false</masked>
      <name>checkout</name>
   </variables>
   <variables>
      <defaultValue>'Lunch'</defaultValue>
      <description></description>
      <id>8d95189f-0e62-4250-bf52-ba3490fa5f9b</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
