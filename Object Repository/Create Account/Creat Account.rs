<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Creat Account</name>
   <tag></tag>
   <elementGuidId>706224ad-7fff-47e7-9875-44ea4bd402a1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;customer_reference\&quot;: \&quot;${randomUserName1}\&quot;,\n    \&quot;name\&quot;: \&quot;${$randomFullName1}\&quot;,\n    \&quot;email\&quot;: \&quot;jones_adelaide@mail.com\&quot;,\n    \&quot;mobile_number\&quot;: \&quot;08012345678\&quot;,\n    \&quot;expires_on\&quot;: \&quot;2021-11-01\&quot;,\n    \&quot;use_frequency\&quot;: \&quot;5\&quot;,\n    \&quot;min_amount\&quot;: 100,\n    \&quot;max_amount\&quot;: 12000,\n    \&quot;callback_url\&quot;: \&quot;https://requesturl.com\&quot;,\n    \&quot;destination_nuban\&quot;: \&quot;\&quot;,\n    \&quot;meta_data\&quot;:{\n    \&quot;somedatakey\&quot;:\&quot;somedatavalue\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_secret</name>
      <type>Main</type>
      <value>vb_ts_bce915031c0dc04a255ec06e380d9621a15dd47fd399</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.woven.finance/v2/api/vnubans/create_customer</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'https://api.woven.finance/'</defaultValue>
      <description>api base url where you can attach the end point</description>
      <id>c61032fd-c09e-474a-a752-f5e7f2f96ea7</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <variables>
      <defaultValue>'vb_ts_bce915031c0dc04a255ec06e380d9621a15dd47fd399'</defaultValue>
      <description>api secret for test authorization</description>
      <id>5133292e-31ab-43e2-a2eb-536a5d4b35cf</id>
      <masked>false</masked>
      <name>api_secret</name>
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
